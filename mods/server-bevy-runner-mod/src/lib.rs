use bevy::{
    app::{ScheduleRunnerPlugin, TerminalCtrlCHandlerPlugin},
    prelude::*,
};
use bevy_mod::BevyMod;
use server_tick_api::ServerTickApi;
use std::time::Duration;
use tokio::task::JoinHandle;

pub struct ServerBevyRunnerMod;

impl ServerBevyRunnerMod {
    pub fn init<T: ServerTickApi>(
        bevy: &mut BevyMod,
        _tick: &mut T,
        _logging: &mut server_bevy_log_mod::ServerBevyLogMod,
    ) -> Self {
        let ticks_per_second = T::ticks_per_second().max(1.0);
        bevy.app
            .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
                Duration::from_secs_f64(1.0 / ticks_per_second),
            )))
            .add_plugins(TerminalCtrlCHandlerPlugin)
            .insert_resource(Time::<Fixed>::from_hz(ticks_per_second));
        info!("server Bevy runner configured at {ticks_per_second:.2} TPS");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
