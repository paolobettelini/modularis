use bevy::prelude::*;
use voxel_math_api::ChunkPos;

#[derive(Resource, Debug, Clone, Copy)]
pub struct ServerChunkResidencyConfig {
    pub horizontal_radius: i32,
    pub vertical_radius: i32,
    pub maintenance_interval_seconds: f32,
}

impl ServerChunkResidencyConfig {
    pub fn contains(self, center: ChunkPos, requested: ChunkPos) -> bool {
        (requested.x - center.x).abs() <= self.horizontal_radius.max(0)
            && (requested.y - center.y).abs() <= self.vertical_radius.max(0)
            && (requested.z - center.z).abs() <= self.horizontal_radius.max(0)
    }
}

pub trait ServerChunkResidencyApi: Send + Sync + 'static {}
