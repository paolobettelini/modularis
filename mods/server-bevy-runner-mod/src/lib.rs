use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_mod::BevyMod;
use std::time::Duration;
use tokio::task::JoinHandle;

pub struct ServerBevyRunnerMod;

impl ServerBevyRunnerMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16))),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
