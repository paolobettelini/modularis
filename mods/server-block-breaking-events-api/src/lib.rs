use bevy::prelude::*;
use block_state_api::BlockState;
use generated_game_mode_registry::GameMode;
use server_chunk_world_api::ResidentChunkKey;
use voxel_frame_api::VoxelBlockAddress;

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ServerValidatedBlockBreak {
    pub player_id: u64,
    pub mode: GameMode,
    pub key: ResidentChunkKey,
    pub position: VoxelBlockAddress,
    pub block: BlockState,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerBlockBreakingSet { Dispatch, ApplyEffects }
