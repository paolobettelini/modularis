use bevy::prelude::*;
use block_instance_api::BlockInstance;
use voxel_math_api::BlockPos;
use world_instance_api::WorldScopeId;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerBlockEditSet {
    Receive,
    Collect,
    Validate,
    Apply,
    Sync,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockBreakRequested {
    pub position: BlockPos,
}

pub type BlockEditorId = u64;

/// Server-side intent after the transport source has been mapped to a player.
/// Keeping the actor allows independent permission, reach, damage and logging
/// mods to validate the request before any world mutation occurs.
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerBlockBreakRequested {
    pub player_id: BlockEditorId,
    pub position: BlockPos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PendingBlockBreak {
    pub player_id: BlockEditorId,
    pub position: BlockPos,
    pub allowed: bool,
}

#[derive(Resource, Default)]
pub struct PendingBlockBreaks {
    pub breaks: Vec<PendingBlockBreak>,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct BlockBroken {
    pub position: BlockPos,
    pub previous: BlockInstance,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct BlockPlaced {
    pub position: BlockPos,
    pub block: BlockInstance,
    pub replaced: BlockInstance,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ServerBlockBroken {
    pub player_id: BlockEditorId,
    pub scope: WorldScopeId,
    pub position: BlockPos,
    pub previous: BlockInstance,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ServerBlockPlaced {
    pub player_id: BlockEditorId,
    pub scope: WorldScopeId,
    pub position: BlockPos,
    pub block: BlockInstance,
    pub replaced: BlockInstance,
}
