use serde::{Deserialize, Serialize};
use voxel_math_api::BlockPos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockDamageProgress { pub position: BlockPos, pub stage: u8 }

