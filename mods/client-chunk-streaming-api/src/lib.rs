use bevy::prelude::*;
use std::collections::HashSet;
use voxel_math_api::ChunkPos;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkNeeded {
    pub position: ChunkPos,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkUnload {
    pub position: ChunkPos,
}

#[derive(Resource, Debug, Default)]
pub struct ActiveChunks {
    pub positions: HashSet<ChunkPos>,
}

#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct ChunkStreamingFocus {
    pub center: Option<ChunkPos>,
}

/// Size of the moving chunk window around the local player. The window has no
/// world-space bounds: it follows the player across every chunk coordinate.
#[derive(Resource, Debug, Clone, Copy)]
pub struct ChunkStreamingViewConfig {
    pub max_horizontal_radius: i32,
    pub vertical_radius: i32,
}

impl Default for ChunkStreamingViewConfig {
    fn default() -> Self {
        Self {
            max_horizontal_radius: 8,
            vertical_radius: 2,
        }
    }
}

pub trait ChunkStreamingApi: Send + Sync + 'static {}
