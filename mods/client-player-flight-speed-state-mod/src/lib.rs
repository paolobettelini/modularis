use bevy_mod::BevyMod;
use player_flight_speed_api::{PlayerFlightSpeedApi, PlayerFlightSpeedMultiplier};
use tokio::task::JoinHandle;

pub struct ClientPlayerFlightSpeedStateMod;

impl ClientPlayerFlightSpeedStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.init_resource::<PlayerFlightSpeedMultiplier>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl PlayerFlightSpeedApi for ClientPlayerFlightSpeedStateMod {}
