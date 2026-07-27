use bevy::prelude::*;
use voxel_math_api::ChunkPos;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GrassChunkMeshRebuilt {
    pub chunk: ChunkPos,
    pub blade_count: usize,
}

pub trait ClientGrassRenderApi: Send + Sync + 'static {}
