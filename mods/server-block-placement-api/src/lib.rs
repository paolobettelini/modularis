use bevy::prelude::*;
use block_state_api::BlockState;
use voxel_frame_api::VoxelBlockAddress;
use inventory_events_api::HeldItemUseDispatched;

/// Validators may only veto. Exactly one apply stage commits accepted intents.
#[derive(Clone,Debug)]
pub struct PendingBlockPlacement {
    pub item_use: HeldItemUseDispatched,
    pub position: VoxelBlockAddress,
    pub block: BlockState,
    pub allowed: bool,
}
#[derive(Resource,Default)]
pub struct PendingBlockPlacements(pub Vec<PendingBlockPlacement>);
#[derive(SystemSet,Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum BlockPlacementSet { Collect, Validate, Apply }
