use block_edit_events_api::{PendingBlockBreak, ServerBlockBreakRequested, ServerBlockBroken};
use server_chunk_world_api::{ServerChunkWorld, WorldEditError};

pub fn allow_block_break(request: &ServerBlockBreakRequested) -> PendingBlockBreak {
    PendingBlockBreak {
        player_id: request.player_id,
        position: request.position,
        allowed: true,
    }
}

/// Applies one already-validated break and converts the world result into the
/// public ECS event contract.
pub fn apply_block_break(
    world: &ServerChunkWorld,
    request: &PendingBlockBreak,
) -> Result<Option<ServerBlockBroken>, WorldEditError> {
    if !request.allowed {
        return Ok(None);
    }
    let mutation = world.break_block_for_player(request.player_id, request.position)?;
    Ok(Some(ServerBlockBroken {
        player_id: request.player_id,
        scope: mutation.scope,
        position: mutation.position,
        previous: mutation.previous,
    }))
}
