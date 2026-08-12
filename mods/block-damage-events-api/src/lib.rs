use bevy::prelude::*;
use voxel_math_api::BlockPos;
use world_instance_api::WorldScopeId;

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ServerBlockDamageChanged {
    pub scope: WorldScopeId,
    pub position: BlockPos,
    /// Zero clears the overlay; positive values are discrete visual stages.
    pub stage: u8,
}

