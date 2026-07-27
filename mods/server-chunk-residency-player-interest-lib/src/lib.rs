use player_network_message_types::NetworkPlayer;
use server_chunk_residency_api::ServerChunkResidencyConfig;
use server_chunk_world_api::{ResidentChunkKey, ServerChunkWorld};
use std::collections::HashSet;
use voxel_math_api::{BlockPos, ChunkPos};

/// Computes the resident chunk set for a selected list of viewers.
///
/// Callers decide which players count as interest sources. A custom server can
/// invoke this independently per node, add NPC or camera interests, or merge it
/// with another residency policy.
pub fn player_interest_chunks(
    world: &ServerChunkWorld,
    players: impl IntoIterator<Item = NetworkPlayer>,
    config: ServerChunkResidencyConfig,
) -> HashSet<ResidentChunkKey> {
    let mut desired = HashSet::new();
    for player in players {
        let center = BlockPos::new(
            player.position[0].floor() as i32,
            player.position[1].floor() as i32,
            player.position[2].floor() as i32,
        )
        .chunk();
        for y in -config.vertical_radius.max(0)..=config.vertical_radius.max(0) {
            for z in -config.horizontal_radius.max(0)..=config.horizontal_radius.max(0) {
                for x in -config.horizontal_radius.max(0)..=config.horizontal_radius.max(0) {
                    let position = ChunkPos::new(center.x + x, center.y + y, center.z + z);
                    if let Some(key) = world.resident_key_for_player(player.id, position) {
                        desired.insert(key);
                    }
                }
            }
        }
    }
    desired
}
