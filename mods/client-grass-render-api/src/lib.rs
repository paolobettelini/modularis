use bevy::prelude::*;
use voxel_math_api::ChunkPos;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GrassChunkMeshRebuilt {
    pub chunk: voxel_frame_api::VoxelChunkAddress,
    pub blade_count: usize,
}

pub trait ClientGrassRenderApi: Send + Sync + 'static {}
