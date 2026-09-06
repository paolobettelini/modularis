use bevy::{math::DVec3, prelude::*};
use std::collections::HashMap;
use voxel_frame_api::*;
use voxel_frame_registry_api::VoxelFrames;
use world_instance_api::WorldScopeId;

#[derive(Resource, Clone, Default)]
pub struct ClientVoxelFrames {
    pub registry: VoxelFrames,
    pub scope: Option<WorldScopeId>,
    /// Logical world coordinates = render coordinates + origin.
    pub render_origin: DVec3,
}
impl ClientVoxelFrames {
    pub fn transform(&self, id: VoxelFrameId) -> Option<VoxelFrameTransform> {
        if id.is_root() { return Some(VoxelFrameTransform::IDENTITY); }
        self.registry.transform(self.scope.as_ref()?,id)
    }
    pub fn upsert(&mut self, frame: VoxelFrame) {
        self.scope = Some(frame.scope.clone());
        let _ = self.registry.upsert(frame);
        self.registry.drain_changes();
    }
    pub fn clear(&mut self) { for frame in self.registry.all(){self.registry.remove(&frame.scope,frame.id);} self.registry.drain_changes(); self.scope = None; }
    pub fn world_chunk(&self, address: VoxelChunkAddress) -> Option<voxel_math_api::ChunkPos> {
        let bounds = VoxelBounds::chunk(address.local);
        let local = DVec3::from_array(bounds.min)+DVec3::splat(8.0);
        let center = self.transform(address.frame)?.local_to_world(local) / 16.0;
        Some(voxel_math_api::ChunkPos::new(center.x.floor() as i32,center.y.floor() as i32,center.z.floor() as i32))
    }
}
#[derive(Resource, Default)]
pub struct ClientVoxelFrameEntities(pub HashMap<VoxelFrameId,Entity>);
impl ClientVoxelFrameEntities {
    pub fn parent(&mut self, commands: &mut Commands, frames: &ClientVoxelFrames, id: VoxelFrameId) -> Option<Entity> {
        if let Some(entity) = self.0.get(&id) { return Some(*entity); }
        let transform = frames.transform(id)?.render_transform(frames.render_origin);
        let entity = commands.spawn((VoxelFrameEntity(id),transform,Visibility::Inherited)).id();
        self.0.insert(id,entity);
        Some(entity)
    }
}

#[derive(SystemSet,Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum ClientVoxelFrameSet { Receive }
