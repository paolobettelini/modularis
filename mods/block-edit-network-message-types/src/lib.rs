use block_state_api::BlockState;
use serde::{Deserialize, Serialize};
use voxel_math_api::BlockPos;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockBreakRequest {
    pub position: BlockPos,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockBrokenPacket {
    pub position: BlockPos,
    pub previous: BlockState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockPlacedPacket {
    pub position: BlockPos,
    pub block: BlockState,
    pub replaced: BlockState,
}
