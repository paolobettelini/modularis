use block_instance_api::BlockInstance;
use serde::{Deserialize, Serialize};
use voxel_math_api::BlockPos;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockBreakRequest {
    pub position: BlockPos,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockBrokenPacket {
    pub position: BlockPos,
    pub previous: BlockInstance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockPlacedPacket {
    pub position: BlockPos,
    pub block: BlockInstance,
    pub replaced: BlockInstance,
}
