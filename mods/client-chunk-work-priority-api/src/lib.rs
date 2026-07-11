use bevy::prelude::*;
use voxel_math_api::ChunkPos;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChunkWorkPriority {
    pub layer: u32,
    pub distance: u64,
}

pub type ChunkWorkPriorityPolicy = fn(ChunkPos, Option<ChunkPos>) -> ChunkWorkPriority;

#[derive(Resource, Clone, Copy)]
pub struct ChunkWorkPriorityService {
    pub priority: ChunkWorkPriorityPolicy,
}

impl Default for ChunkWorkPriorityService {
    fn default() -> Self {
        Self {
            priority: |_position, _focus| ChunkWorkPriority::default(),
        }
    }
}

pub trait ClientChunkWorkPriorityApi: Send + Sync + 'static {}
