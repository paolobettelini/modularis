use bevy::prelude::*;
use bevy_mod::BevyMod;
use player_flight_api::{
    FlightConfig, LocalFlightCapabilityChanged, LocalPlayerFlight, PlayerFlightApi,
};
use tokio::task::JoinHandle;

pub struct ClientPlayerFlightStateMod;

impl ClientPlayerFlightStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .init_resource::<LocalPlayerFlight>()
            .init_resource::<FlightConfig>()
            .add_message::<LocalFlightCapabilityChanged>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl PlayerFlightApi for ClientPlayerFlightStateMod {}
