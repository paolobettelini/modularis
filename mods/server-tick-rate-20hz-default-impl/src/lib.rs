use bevy_mod::BevyMod;
use server_tick_api::{ServerTickApi, ServerTickRate};
use tokio::task::JoinHandle;

pub struct ServerTickRate20HzDefaultImpl;

impl ServerTickRate20HzDefaultImpl {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.insert_resource(ServerTickRate {
            target_tps: Self::ticks_per_second(),
        });
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerTickApi for ServerTickRate20HzDefaultImpl {
    fn ticks_per_second() -> f64 {
        20.0
    }
}
