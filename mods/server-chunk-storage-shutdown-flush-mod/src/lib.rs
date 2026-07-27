use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_bevy_runner_mod::ServerBevyRunnerMod;
use server_chunk_storage_api::{ServerChunkStorage, ServerChunkStorageApi};
use tokio::task::JoinHandle;

pub struct ServerChunkStorageShutdownFlushMod;

impl ServerChunkStorageShutdownFlushMod {
    pub fn init<S: ServerChunkStorageApi>(
        bevy: &mut BevyMod,
        _runner: &mut ServerBevyRunnerMod,
        _storage_api: &mut S,
    ) -> Self {
        bevy.app.add_systems(Last, flush_chunk_storage_on_exit);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn flush_chunk_storage_on_exit(
    mut exits: MessageReader<AppExit>,
    storage: Res<ServerChunkStorage>,
) {
    if exits.read().next().is_none() {
        return;
    }
    match storage.flush() {
        Ok(report) => info!(
            "shutdown chunk flush wrote {} chunks across {} world regions",
            report.chunks_written, report.regions_written
        ),
        Err(error) => error!("shutdown chunk storage flush failed: {error}"),
    }
}
