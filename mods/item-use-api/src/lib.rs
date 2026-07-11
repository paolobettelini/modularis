use serde::{Deserialize, Serialize};
use voxel_math_api::BlockPos;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemUseTarget {
    None,
    Block {
        hit: BlockPos,
        adjacent: BlockPos,
        normal: [i32; 3],
    },
}
