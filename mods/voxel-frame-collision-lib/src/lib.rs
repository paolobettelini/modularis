use bevy::{prelude::*,math::DVec3};
use collision_api::*;
use voxel_frame_api::*;
use voxel_math_api::BlockPos;
use block_shape_api::BlockShape;
/// Reuses existing local model boxes. Consumers provide scoped broadphase and data access.
pub struct VoxelGeometry<F,Q> {pub shape:F,pub frames:Q}
impl<F,Q> CharacterGeometry for VoxelGeometry<F,Q>
where F:Fn(VoxelBlockAddress)->BlockShape+Send+Sync,Q:Fn(VoxelBounds)->Vec<VoxelFrame>+Send+Sync {
 fn query(&self,bounds:Aabb)->Vec<CollisionBox>{
  let world=VoxelBounds{min:bounds.min.as_dvec3().to_array(),max:bounds.max.as_dvec3().to_array()};
  let frames=(self.frames)(world);
  let mut domains=vec![(VoxelFrameId::ROOT,VoxelFrameTransform::IDENTITY,None)];
  domains.extend(frames.into_iter().map(|f|(f.id,f.transform,Some(f.occupied_chunks))));
  let mut result=Vec::new();
  for (id,pose,occupied) in domains{
   let inverse=VoxelFrameTransform::new(pose.world_to_local(DVec3::ZERO).to_array(),pose.rotation().conjugate().to_array()).unwrap();
   let local=inverse.transform_bounds(world);
   let min=DVec3::from_array(local.min).floor()-DVec3::ONE;
   let max=DVec3::from_array(local.max).ceil()+DVec3::ONE;
   for y in min.y as i32..max.y as i32 {for z in min.z as i32..max.z as i32 {for x in min.x as i32..max.x as i32 {
    let address=VoxelBlockAddress::new(id,BlockPos::new(x,y,z));
    if occupied.as_ref().is_some_and(|set|!set.contains(&address.chunk().local)){continue;}
    for b in (self.shape)(address).boxes(){
     let center=Vec3::new(x as f32,y as f32,z as f32)+(b.min+b.max)*0.5;
     result.push(CollisionBox{center:pose.local_to_world(center.as_dvec3()).as_vec3(),rotation:pose.rotation().as_quat(),half_extents:(b.max-b.min)*0.5,surface:id.0.as_u128()});
    }
   }}}
  }result
 }
}
