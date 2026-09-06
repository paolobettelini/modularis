use chunk_api::Chunk;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChunkRequest {
    pub movement_epoch: u64,
    pub position: voxel_frame_api::VoxelChunkAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkResponse {
    pub movement_epoch: u64,
    pub chunk: Chunk,
    pub frame: voxel_frame_api::VoxelFrameId,
}
