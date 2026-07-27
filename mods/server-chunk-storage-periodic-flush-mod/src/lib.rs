use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_bevy_runner_mod::ServerBevyRunnerMod;
use server_chunk_storage_api::{
    ChunkStorageFlushInterval, ServerChunkStorage, ServerChunkStorageApi,
};
use tokio::task::JoinHandle;

#[derive(Resource)]
struct ChunkStorageFlushTimer(Timer);

pub struct ServerChunkStoragePeriodicFlushMod;

impl ServerChunkStoragePeriodicFlushMod {
    pub fn init<S: ServerChunkStorageApi>(
        bevy: &mut BevyMod,
        _runner: &mut ServerBevyRunnerMod,
        _storage_api: &mut S,
    ) -> Self {
        if !bevy
            .app
            .world()
            .contains_resource::<ChunkStorageFlushInterval>()
        {
            bevy.app
                .insert_resource(ChunkStorageFlushInterval::default());
        }
        let interval = bevy.app.world().resource::<ChunkStorageFlushInterval>().0;
        bevy.app
            .insert_resource(ChunkStorageFlushTimer(Timer::new(
                interval,
                TimerMode::Repeating,
            )))
            .add_systems(Update, flush_chunk_storage_periodically);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn flush_chunk_storage_periodically(
    time: Res<Time>,
    mut timer: ResMut<ChunkStorageFlushTimer>,
    storage: Res<ServerChunkStorage>,
) {
    if !timer.0.tick(time.delta()).just_finished() || storage.pending_chunks() == 0 {
        return;
    }
    match storage.flush() {
        Ok(report) if report.regions_written > 0 => info!(
            "flushed {} chunks across {} world regions",
            report.chunks_written, report.regions_written
        ),
        Ok(_) => {}
        Err(error) => error!("periodic chunk storage flush failed: {error}"),
    }
}
