use chunk_api::Chunk;
use serde::{Deserialize, Serialize};
use voxel_math_api::ChunkPos;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChunkRequest {
    pub position: ChunkPos,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkResponse {
    pub chunk: Chunk,
}
