use bevy_mod::BevyMod;
use player_speed_api::{PlayerSpeedApi, PlayerSpeedMultiplier};
use tokio::task::JoinHandle;

pub struct ClientPlayerSpeedStateMod;

impl ClientPlayerSpeedStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.init_resource::<PlayerSpeedMultiplier>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl PlayerSpeedApi for ClientPlayerSpeedStateMod {}
