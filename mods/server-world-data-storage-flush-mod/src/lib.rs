use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_bevy_runner_mod::ServerBevyRunnerMod;
use server_world_data_storage_api::{ServerWorldDataStorage, ServerWorldDataStorageApi, WorldDataFlushInterval};
use tokio::task::JoinHandle;

#[derive(Resource)]
struct FlushTimer(Timer);

pub struct ServerWorldDataStorageFlushMod;
impl ServerWorldDataStorageFlushMod {
    pub fn init<S: ServerWorldDataStorageApi>(bevy: &mut BevyMod, _runner: &mut ServerBevyRunnerMod, _storage: &mut S) -> Self {
        if !bevy.app.world().contains_resource::<WorldDataFlushInterval>() { bevy.app.insert_resource(WorldDataFlushInterval::default()); }
        let interval = bevy.app.world().resource::<WorldDataFlushInterval>().0;
        bevy.app.insert_resource(FlushTimer(Timer::new(interval, TimerMode::Repeating)))
            .add_systems(Update, periodic_flush).add_systems(Last, shutdown_flush);
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn periodic_flush(time: Res<Time>, mut timer: ResMut<FlushTimer>, storage: Res<ServerWorldDataStorage>) {
    if timer.0.tick(time.delta()).just_finished() && storage.pending_records() > 0 { flush(&storage, "periodic"); }
}
fn shutdown_flush(mut exits: MessageReader<AppExit>, storage: Res<ServerWorldDataStorage>) {
    if exits.read().next().is_some() { flush(&storage, "shutdown"); }
}
fn flush(storage: &ServerWorldDataStorage, reason: &str) {
    match storage.flush() {
        Ok(report) if report.records_written + report.records_deleted > 0 => info!("{reason} world data flush wrote {} and deleted {} records", report.records_written, report.records_deleted),
        Ok(_) => {}
        Err(error) => error!("{reason} world data flush failed: {error}"),
    }
}

