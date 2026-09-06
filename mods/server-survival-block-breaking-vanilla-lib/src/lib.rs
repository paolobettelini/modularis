use block_state_api::BlockState;
use generated_game_mode_registry::GameMode;
use server_block_breaking_events_api::ServerValidatedBlockBreak;
use server_chunk_world_api::ResidentChunkKey;
use std::collections::{HashMap, HashSet};
use voxel_frame_api::VoxelBlockAddress;

#[derive(Debug, Clone)]
pub struct SurvivalDamageBatch {
    pub key: ResidentChunkKey,
    pub position: VoxelBlockAddress,
    pub block: BlockState,
    pub players: HashSet<u64>,
}

/// Deduplicates repeated packets from one player in a server tick while
/// preserving additive contribution from distinct players.
pub fn collect_survival_damage<'a>(
    requests: impl IntoIterator<Item = &'a ServerValidatedBlockBreak>,
) -> Vec<SurvivalDamageBatch> {
    let mut batches = HashMap::<(ResidentChunkKey, VoxelBlockAddress), SurvivalDamageBatch>::new();
    for request in requests {
        if request.mode != GameMode::Survival { continue; }
        let entry = batches.entry((request.key.clone(), request.position)).or_insert_with(|| SurvivalDamageBatch {
            key: request.key.clone(), position: request.position, block: request.block.clone(), players: HashSet::new(),
        });
        entry.players.insert(request.player_id);
    }
    batches.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use block_state_api::BlockId;
    use server_chunk_provider_api::ChunkProviderId;
    use voxel_math_api::ChunkPos;
    use world_instance_api::WorldInstanceId;

    fn request(player_id: u64, mode: GameMode) -> ServerValidatedBlockBreak {
        let position = VoxelBlockAddress::root(voxel_math_api::BlockPos::new(1, 2, 3));
        ServerValidatedBlockBreak {
            player_id,
            mode,
            key: ResidentChunkKey {
                frame: voxel_frame_api::VoxelFrameId::ROOT,
                instance: WorldInstanceId::new("test:world"),
                provider: ChunkProviderId::new("test:terrain"),
                position: ChunkPos::new(0, 0, 0),
            },
            position,
            block: BlockId::Stone.into(),
        }
    }

    #[test]
    fn survival_batches_distinct_players_and_ignores_other_modes() {
        let requests = [
            request(1, GameMode::Survival),
            request(1, GameMode::Survival),
            request(2, GameMode::Survival),
            request(3, GameMode::Creative),
        ];
        let batches = collect_survival_damage(&requests);

        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0].players, HashSet::from([1, 2]));
    }
}
