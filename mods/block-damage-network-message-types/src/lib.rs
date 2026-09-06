use serde::{Deserialize, Serialize};
use voxel_frame_api::VoxelBlockAddress;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockDamageProgress { pub position: VoxelBlockAddress, pub stage: u8 }
