use client_player_physics_tick_api::ClientPlayerPhysicsTickApi;
use tokio::task::JoinHandle;

pub struct ClientPlayerPhysicsTick20HzVanillaMod;

impl ClientPlayerPhysicsTick20HzVanillaMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientPlayerPhysicsTickApi for ClientPlayerPhysicsTick20HzVanillaMod {
    fn ticks_per_second() -> f64 {
        20.0
    }
}
