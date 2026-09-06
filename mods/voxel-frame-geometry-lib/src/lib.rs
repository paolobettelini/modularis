use bevy::{math::DVec3,prelude::*};
use voxel_frame_api::*;
use voxel_math_api::BlockPos;
use block_shape_api::BlockShape;
use collision_api::Aabb;

#[derive(Debug,Clone,Copy)]
pub struct VoxelFrameRayHit {
    pub block: VoxelBlockAddress,
    pub adjacent: VoxelBlockAddress,
    /// Discrete face direction in the selected frame, not in world space.
    pub normal: IVec3,
    pub world_normal: DVec3,
    pub world_position: DVec3,
    pub distance: f64,
}
pub fn ray_bounds(origin:DVec3,direction:DVec3,reach:f64)->VoxelBounds {
    let end=origin+direction.normalize_or_zero()*reach;
    VoxelBounds{min:(origin.min(end)-DVec3::splat(1e-6)).to_array(),max:(origin.max(end)+DVec3::splat(1e-6)).to_array()}
}
/// The implicit root uses the original local-grid DDA fast path.
pub fn raycast(
    origin:DVec3,direction:DVec3,reach:f64,
    candidates:impl IntoIterator<Item=VoxelFrame>,
    mut shape_at:impl FnMut(VoxelBlockAddress)->BlockShape,
)->Option<VoxelFrameRayHit> {
    if !origin.is_finite() || !direction.is_finite() || !reach.is_finite() || reach<=0.0 {return None;}
    let direction=direction.normalize_or_zero();
    if direction==DVec3::ZERO {return None;}
    let mut best=cast_local(VoxelFrameId::ROOT,VoxelFrameTransform::IDENTITY,origin,direction,reach,&mut shape_at);
    for frame in candidates {
        if frame.id.is_root(){continue;}
        let limit=best.map_or(reach,|h|h.distance);
        if frame.world_bounds().and_then(|b|b.ray_interval(origin,direction,limit)).is_none(){continue;}
        if let Some(hit)=cast_local(frame.id,frame.transform,origin,direction,limit,&mut shape_at) {
            if best.is_none_or(|b|hit.distance<b.distance || (hit.distance==b.distance && hit.block.frame<b.block.frame)) {best=Some(hit);}
        }
    }
    best
}
fn cast_local(id:VoxelFrameId,pose:VoxelFrameTransform,origin:DVec3,direction:DVec3,reach:f64,shape_at:&mut impl FnMut(VoxelBlockAddress)->BlockShape)->Option<VoxelFrameRayHit> {
    let local_origin=pose.world_to_local(origin);
    let local_direction=pose.world_direction_to_local(direction);
    let hit=voxel_raycast_api::raycast_voxel_shapes(local_origin.as_vec3(),local_direction.as_vec3(),reach as f32,
        |local|shape_at(VoxelBlockAddress::new(id,local)))?;
    Some(VoxelFrameRayHit{block:VoxelBlockAddress::new(id,hit.block),adjacent:VoxelBlockAddress::new(id,hit.adjacent),normal:hit.normal,
        world_normal:pose.local_direction_to_world(hit.normal.as_dvec3()),world_position:origin+direction*hit.distance as f64,distance:hit.distance as f64})
}
pub fn block_bounds(position:BlockPos,bounds:Aabb)->VoxelBounds {
    let offset=DVec3::new(position.x as f64,position.y as f64,position.z as f64);
    VoxelBounds{min:(offset+bounds.min.as_dvec3()).to_array(),max:(offset+bounds.max.as_dvec3()).to_array()}
}
#[derive(Debug,Clone,Copy)]
pub struct OrientedVoxelBox { pub center:DVec3,pub half:DVec3,pub axes:[DVec3;3] }
impl OrientedVoxelBox {
    pub fn new(bounds:VoxelBounds,pose:VoxelFrameTransform)->Self {
        let min=DVec3::from_array(bounds.min); let max=DVec3::from_array(bounds.max);
        Self{center:pose.local_to_world((min+max)*0.5),half:(max-min)*0.5,axes:[DVec3::X,DVec3::Y,DVec3::Z].map(|v|pose.local_direction_to_world(v))}
    }
    /// SAT over both face normals and all edge cross products. Touching faces
    /// are allowed; this is overlap validation, not contact resolution.
    pub fn overlaps(self,other:Self)->bool {
        let d=other.center-self.center;
        let axes=self.axes.into_iter().chain(other.axes).chain(self.axes.into_iter().flat_map(|a|other.axes.map(|b|a.cross(b))));
        for axis in axes {
            if axis.length_squared()<1e-18 {continue;}
            let axis=axis.normalize();
            let a=(0..3).map(|i|self.half[i]*self.axes[i].dot(axis).abs()).sum::<f64>();
            let b=(0..3).map(|i|other.half[i]*other.axes[i].dot(axis).abs()).sum::<f64>();
            if d.dot(axis).abs()>=a+b-1e-7 {return false;}
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn sat_allows_contact_but_rejects_overlap() {
        let b=VoxelBounds{min:[0.0;3],max:[1.0;3]};
        let a=OrientedVoxelBox::new(b,VoxelFrameTransform::IDENTITY);
        let pose=|x|VoxelFrameTransform::new([x,0.0,0.0],[0.0,0.0,0.0,1.0]).unwrap();
        assert!(!a.overlaps(OrientedVoxelBox::new(b,pose(1.0))));
        assert!(a.overlaps(OrientedVoxelBox::new(b,pose(0.5))));
    }
    #[test] fn rotated_ray_returns_local_adjacency_and_world_hit() {
        let id=VoxelFrameId::new();
        let pose=VoxelFrameTransform::new([9.0,12.0,-3.0],bevy::math::DQuat::from_euler(EulerRot::XYZ,0.2,0.4,0.1).to_array()).unwrap();
        let mut frame=VoxelFrame::new(id,world_instance_api::WorldScopeId::new(world_instance_api::WorldInstanceId::new("test"),"test:provider"),pose);
        frame.occupied_chunks.insert(voxel_math_api::ChunkPos::new(0,0,0));
        let origin=pose.local_to_world(DVec3::new(-2.0,0.5,0.5));
        let direction=pose.local_direction_to_world(DVec3::X);
        let hit=raycast(origin,direction,6.0,[frame],|address| {
            if address==VoxelBlockAddress::new(id,BlockPos::new(0,0,0)) { BlockShape::new(vec![Aabb{min:Vec3::ZERO,max:Vec3::ONE}]) } else { BlockShape::empty() }
        }).unwrap();
        assert_eq!(hit.block,VoxelBlockAddress::new(id,BlockPos::new(0,0,0)));
        assert_eq!(hit.adjacent,VoxelBlockAddress::new(id,BlockPos::new(-1,0,0)));
        assert!(hit.world_position.distance(pose.local_to_world(DVec3::new(0.0,0.5,0.5)))<1e-5);
    }

}
