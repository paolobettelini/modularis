use serde::{Deserialize, Serialize};
use voxel_frame_api::VoxelBlockAddress;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemUseTarget {
    None,
    Block {
        hit: VoxelBlockAddress,
        adjacent: VoxelBlockAddress,
        normal: [i32; 3],
    },
}
