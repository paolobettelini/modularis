use bevy::prelude::*;
use voxel_math_api::ChunkPos;
use chunk_interest_api::{ChunkInterest, SphericalChunkInterest};

#[derive(Resource, Clone)]
pub struct ServerChunkResidencyConfig {
    pub radius: i32,
    pub volume: ChunkInterest,
    pub maintenance_interval_seconds: f32,
}
impl Default for ServerChunkResidencyConfig {
    fn default() -> Self { Self { radius: 9, volume: ChunkInterest::new(SphericalChunkInterest), maintenance_interval_seconds: 1.0 } }
}
impl ServerChunkResidencyConfig {
    pub fn contains(&self, center: ChunkPos, requested: ChunkPos) -> bool {
        self.volume.contains(center,requested,self.radius)
    }
}
pub trait ServerChunkResidencyApi: Send + Sync + 'static {}
