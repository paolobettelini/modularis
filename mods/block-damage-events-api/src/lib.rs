use bevy::prelude::*;
use voxel_frame_api::VoxelBlockAddress;
use world_instance_api::WorldScopeId;

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ServerBlockDamageChanged {
    pub scope: WorldScopeId,
    pub position: VoxelBlockAddress,
    /// Zero clears the overlay; positive values are discrete visual stages.
    pub stage: u8,
}
