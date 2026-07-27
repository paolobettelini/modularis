use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_chunk_residency_api::{ServerChunkResidencyApi, ServerChunkResidencyConfig};
use server_chunk_residency_player_interest_lib::player_interest_chunks;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_player_registry_api::{
    ServerPlayerMovementSet, ServerPlayerRegistry, ServerPlayerRegistryApi,
};
use tokio::task::JoinHandle;

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

    let desired = player_interest_chunks(&world, players.players(), *config);
    world.retain_resident(&desired);
}
