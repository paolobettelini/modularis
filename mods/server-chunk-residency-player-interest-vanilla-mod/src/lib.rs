use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_chunk_residency_api::{ServerChunkResidencyApi, ServerChunkResidencyConfig};
use server_chunk_world_api::{ResidentChunkKey, ServerChunkWorld, ServerChunkWorldApi};
use server_player_registry_api::{
    ServerPlayerMovementSet, ServerPlayerRegistry, ServerPlayerRegistryApi,
};
use std::collections::HashSet;
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, ChunkPos};

#[derive(Resource)]
struct ChunkResidencyMaintenanceTimer(Timer);

pub struct ServerChunkResidencyPlayerInterestVanillaMod;

impl ServerChunkResidencyPlayerInterestVanillaMod {
    pub fn init<W: ServerChunkWorldApi, P: ServerPlayerRegistryApi>(
        bevy: &mut BevyMod,
        _world: &mut W,
        _players: &mut P,
    ) -> Self {
        let config = ServerChunkResidencyConfig {
            // Keep one chunk of slack beyond the client's maximum radius so
            // requests at a chunk boundary are not rejected while the latest
            // predicted player position is still in transit.
            horizontal_radius: 9,
            // The client keeps two chunks above/below its current chunk. Keep
            // one extra chunk so boundary requests survive movement latency.
            vertical_radius: 3,
            maintenance_interval_seconds: 1.0,
        };
        bevy.app
            .insert_resource(config)
            .insert_resource(ChunkResidencyMaintenanceTimer(Timer::from_seconds(
                config.maintenance_interval_seconds,
                TimerMode::Repeating,
            )))
            .add_systems(
                Update,
                maintain_player_chunk_residency.after(ServerPlayerMovementSet::Apply),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerChunkResidencyApi for ServerChunkResidencyPlayerInterestVanillaMod {}

fn maintain_player_chunk_residency(
    time: Res<Time>,
    config: Res<ServerChunkResidencyConfig>,
    mut timer: ResMut<ChunkResidencyMaintenanceTimer>,
    players: Res<ServerPlayerRegistry>,
    world: Res<ServerChunkWorld>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let mut desired = HashSet::<ResidentChunkKey>::new();
    for player in players.players() {
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
    world.retain_resident(&desired);
}
