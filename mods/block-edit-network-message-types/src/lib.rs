use block_state_api::BlockState;
use serde::{Deserialize, Serialize};
use voxel_frame_api::VoxelBlockAddress;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockBreakRequest {
    pub position: VoxelBlockAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockBrokenPacket {
    pub position: VoxelBlockAddress,
    pub previous: BlockState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockPlacedPacket {
    pub position: VoxelBlockAddress,
    pub block: BlockState,
    pub replaced: BlockState,
}
