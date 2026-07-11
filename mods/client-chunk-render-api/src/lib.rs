use bevy::prelude::*;
use std::collections::HashMap;
use voxel_math_api::ChunkPos;

#[derive(Resource, Default)]
pub struct RenderedChunks {
    pub entities: HashMap<ChunkPos, Vec<Entity>>,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct ChunkRemeshBudget {
    pub chunks_per_frame: usize,
}

impl Default for ChunkRemeshBudget {
    fn default() -> Self {
        Self {
            chunks_per_frame: 4,
        }
    }
}

pub trait ChunkRenderApi: Send + Sync + 'static {}
