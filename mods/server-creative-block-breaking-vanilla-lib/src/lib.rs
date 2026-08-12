use block_edit_events_api::{PendingBlockBreak, ServerBlockBroken};
use generated_game_mode_registry::GameMode;
use server_block_breaking_events_api::ServerValidatedBlockBreak;
use server_block_edit_world_lib::apply_block_break;
use server_chunk_world_api::{ServerChunkWorld, WorldEditError};

pub fn apply_creative_break(
    world: &ServerChunkWorld,
    request: &ServerValidatedBlockBreak,
) -> Result<Option<ServerBlockBroken>, WorldEditError> {
    if request.mode != GameMode::Creative { return Ok(None); }
    apply_block_break(world, &PendingBlockBreak {
        player_id: request.player_id,
        position: request.position,
        allowed: true,
    })
}

