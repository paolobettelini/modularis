use bevy_mod::BevyMod;
use server_bevy_runner_mod::ServerBevyRunnerMod;
use tokio::task::JoinHandle;

pub struct ServerGameBootstrapMod;

impl ServerGameBootstrapMod {
    pub fn init(_bevy: &mut BevyMod, _runner: &mut ServerBevyRunnerMod) -> Self {
        Self
    }

    pub fn run(&self, mut bevy: BevyMod) -> Option<Vec<JoinHandle<()>>> {
        bevy.app.run();
        None
    }
}
