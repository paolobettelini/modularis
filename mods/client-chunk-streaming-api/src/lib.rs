use bevy::prelude::*;
use std::collections::HashSet;
use voxel_math_api::ChunkPos;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkNeeded {
    pub position: voxel_frame_api::VoxelChunkAddress,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkUnload {
    pub position: voxel_frame_api::VoxelChunkAddress,
}

#[derive(Resource, Debug, Default)]
pub struct ActiveChunks {
    pub positions: HashSet<voxel_frame_api::VoxelChunkAddress>,
}

#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct ChunkStreamingFocus {
    pub center: Option<ChunkPos>,
}

/// Size of the moving chunk window around the local player. The window has no
/// world-space bounds: it follows the player across every chunk coordinate.
#[derive(Resource, Clone)]
pub struct ChunkStreamingViewConfig {
    pub max_radius: i32,
    pub volume: chunk_interest_api::ChunkInterest,
}

impl Default for ChunkStreamingViewConfig {
    fn default() -> Self {
        Self {
            max_radius: 8,
            volume: chunk_interest_api::ChunkInterest::new(chunk_interest_api::SphericalChunkInterest),
        }
    }
}

pub trait ChunkStreamingApi: Send + Sync + 'static {}
