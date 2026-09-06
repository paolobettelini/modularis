//! Spatial identities and rigid transforms. Block/chunk payloads remain local.
use bevy::{math::{DVec3, DQuat}, prelude::*};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use voxel_math_api::{BlockPos, ChunkPos, LocalBlockPos, CHUNK_SIZE};
use world_instance_api::WorldScopeId;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VoxelFrameId(pub uuid::Uuid);
impl VoxelFrameId {
    /// Implicit identity frame in each independently routed world scope.
    pub const ROOT: Self = Self(uuid::Uuid::nil());
    pub fn from_u128(id:u128)->Self {Self(uuid::Uuid::from_u128(id))}
    pub fn new() -> Self { Self(uuid::Uuid::new_v4()) }
    pub fn is_root(self) -> bool { self == Self::ROOT }
}
impl std::fmt::Display for VoxelFrameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VoxelBlockAddress {
    pub frame: VoxelFrameId,
    pub local: BlockPos,
}
impl VoxelBlockAddress {
    pub const fn new(frame: VoxelFrameId, local: BlockPos) -> Self { Self { frame, local } }
    pub const fn root(local: BlockPos) -> Self { Self::new(VoxelFrameId::ROOT, local) }
    pub fn chunk(self) -> VoxelChunkAddress { VoxelChunkAddress::new(self.frame, self.local.chunk()) }
    pub fn local(self) -> LocalBlockPos { self.local.local() }
}
impl From<BlockPos> for VoxelBlockAddress { fn from(local: BlockPos) -> Self { Self::root(local) } }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VoxelChunkAddress {
    pub frame: VoxelFrameId,
    pub local: ChunkPos,
}
impl VoxelChunkAddress {
    pub const fn new(frame: VoxelFrameId, local: ChunkPos) -> Self { Self { frame, local } }
    pub const fn root(local: ChunkPos) -> Self { Self::new(VoxelFrameId::ROOT, local) }
    pub fn block(self, local: LocalBlockPos) -> VoxelBlockAddress { VoxelBlockAddress::new(self.frame, local.to_world(self.local)) }
}
impl From<ChunkPos> for VoxelChunkAddress { fn from(local: ChunkPos) -> Self { Self::root(local) } }

/// Serialized rigid pose: private components prevent invalid public construction.
/// Double precision logical translation is independent from renderer origin.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct VoxelFrameTransform {
    translation: [f64; 3],
    rotation: [f64; 4],
}
impl<'de> Deserialize<'de> for VoxelFrameTransform {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)] struct Pose { translation: [f64; 3], rotation: [f64; 4] }
        let pose = Pose::deserialize(deserializer)?;
        Self::new(pose.translation, pose.rotation).map_err(serde::de::Error::custom)
    }
}
impl Default for VoxelFrameTransform { fn default() -> Self { Self::IDENTITY } }
impl VoxelFrameTransform {
    pub const IDENTITY: Self = Self { translation: [0.0; 3], rotation: [0.0, 0.0, 0.0, 1.0] };
    pub fn new(translation: [f64; 3], rotation: [f64; 4]) -> Result<Self, &'static str> {
        let q = DQuat::from_array(rotation);
        if !DVec3::from_array(translation).is_finite() || !q.is_finite() || !q.length_squared().is_finite() || q.length_squared() < 1e-16 {
            return Err("frame pose must have finite translation and a nonzero quaternion");
        }
        Ok(Self { translation, rotation: q.normalize().to_array() })
    }
    pub fn translation(self) -> DVec3 { DVec3::from_array(self.translation) }
    pub fn rotation(self) -> DQuat { DQuat::from_array(self.rotation) }
    pub fn local_to_world(self, p: DVec3) -> DVec3 { self.rotation() * p + self.translation() }
    pub fn world_to_local(self, p: DVec3) -> DVec3 { self.rotation().conjugate() * (p - self.translation()) }
    pub fn local_direction_to_world(self, d: DVec3) -> DVec3 { self.rotation() * d }
    pub fn world_direction_to_local(self, d: DVec3) -> DVec3 { self.rotation().conjugate() * d }
    pub fn render_transform(self, render_origin: DVec3) -> Transform {
        Transform::from_translation((self.translation() - render_origin).as_vec3())
            .with_rotation(Quat::from_xyzw(self.rotation[0] as f32, self.rotation[1] as f32, self.rotation[2] as f32, self.rotation[3] as f32))
    }
    pub fn transform_bounds(self, bounds: VoxelBounds) -> VoxelBounds {
        let mut min = DVec3::splat(f64::INFINITY);
        let mut max = DVec3::splat(f64::NEG_INFINITY);
        for x in [bounds.min[0], bounds.max[0]] { for y in [bounds.min[1], bounds.max[1]] { for z in [bounds.min[2], bounds.max[2]] {
            let p = self.local_to_world(DVec3::new(x,y,z)); min = min.min(p); max = max.max(p);
        }}}
        VoxelBounds { min: min.to_array(), max: max.to_array() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VoxelBounds { pub min: [f64; 3], pub max: [f64; 3] }
impl VoxelBounds {
    pub fn chunk(p: ChunkPos) -> Self {
        let min = DVec3::new(p.x as f64, p.y as f64, p.z as f64) * CHUNK_SIZE as f64;
        Self { min: min.to_array(), max: (min + DVec3::splat(CHUNK_SIZE as f64)).to_array() }
    }
    pub fn overlaps(self, b: Self) -> bool { (0..3).all(|a| self.min[a] < b.max[a] && self.max[a] > b.min[a]) }
    pub fn union(self, b: Self) -> Self {
        Self { min: std::array::from_fn(|i| self.min[i].min(b.min[i])), max: std::array::from_fn(|i| self.max[i].max(b.max[i])) }
    }
    pub fn ray_interval(self, origin: DVec3, direction: DVec3, reach: f64) -> Option<(f64, f64)> {
        let (mut near, mut far) = (0.0_f64, reach);
        for i in 0..3 {
            if direction[i].abs() < 1e-12 {
                if origin[i] < self.min[i] || origin[i] > self.max[i] { return None; }
            } else {
                let a = (self.min[i]-origin[i])/direction[i];
                let b = (self.max[i]-origin[i])/direction[i];
                near = near.max(a.min(b)); far = far.min(a.max(b));
                if near > far { return None; }
            }
        }
        Some((near, far))
    }
}

/// The root is implicit. Explicit frames enumerate only their occupied chunks.
/// A frame is scoped to a world route, but is not itself a world/provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoxelFrame {
    pub id: VoxelFrameId,
    pub scope: WorldScopeId,
    pub transform: VoxelFrameTransform,
    pub occupied_chunks: BTreeSet<ChunkPos>,
    pub revision: u64,
}
impl VoxelFrame {
    pub fn new(id: VoxelFrameId, scope: WorldScopeId, transform: VoxelFrameTransform) -> Self {
        Self { id, scope, transform, occupied_chunks: BTreeSet::new(), revision: 1 }
    }
    pub fn local_bounds(&self) -> Option<VoxelBounds> {
        self.occupied_chunks.iter().copied().map(VoxelBounds::chunk).reduce(VoxelBounds::union)
    }
    pub fn world_bounds(&self) -> Option<VoxelBounds> { self.local_bounds().map(|b| self.transform.transform_bounds(b)) }
}

/// A renderer/physics provider can attach this to one entity per frame.
/// Updating its pose does not alter local voxel addresses.
#[derive(Component, Debug, Clone, Copy)]
pub struct VoxelFrameEntity(pub VoxelFrameId);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn local_addresses_and_rigid_roundtrip() {
        let a = VoxelBlockAddress::new(VoxelFrameId::new(), BlockPos::new(-17,-1,33));
        let b = VoxelBlockAddress::new(VoxelFrameId::new(), a.local);
        assert_ne!(a,b);
        assert_eq!(a.chunk().local, ChunkPos::new(-2,-1,2));
        assert_eq!(a.chunk().block(a.local()), a);
        let pose = VoxelFrameTransform::new([12.0,-2.0,50.0], DQuat::from_euler(bevy::math::EulerRot::XYZ,0.4,0.2,0.7).to_array()).unwrap();
        let p = DVec3::new(-17.25,3.0,19.75);
        assert!(pose.world_to_local(pose.local_to_world(p)).distance(p) < 1e-10);
        assert_eq!(VoxelFrameTransform::IDENTITY.local_to_world(p),p);
        assert_eq!(a.local, b.local);
    }
}

/// Optional physics adapters attach their own components to a frame entity.
/// Collision policy is independent from whether a placement overlaps.
pub trait VoxelFrameCollisionPolicy: Send + Sync + 'static {
    fn can_collide(&self, left: VoxelFrameId, right: VoxelFrameId) -> bool;
}
#[derive(Component,Debug,Clone,Copy,Default)]
pub struct VoxelFrameVelocity {
    pub linear_world: [f64;3],
    pub angular_world: [f64;3],
}
