use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::{NetworkMessageSet, PlayerFlightSpeedChangedReceived};
use network_protocol_mod::NetworkProtocolMod;
use player_flight_speed_api::{PlayerFlightSpeedApi, PlayerFlightSpeedMultiplier};
use tokio::task::JoinHandle;

pub struct ClientPlayerFlightSpeedNetworkReceiveMod;

impl ClientPlayerFlightSpeedNetworkReceiveMod {
    pub fn init<S: PlayerFlightSpeedApi>(
        bevy: &mut BevyMod,
        _speed: &mut S,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            apply_server_flight_speed.after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_server_flight_speed(
    mut messages: MessageReader<PlayerFlightSpeedChangedReceived>,
    mut speed: ResMut<PlayerFlightSpeedMultiplier>,
) {
    for message in messages.read() {
        if message.0.multiplier.is_finite() {
            speed.0 = message.0.multiplier.max(0.0);
        }
    }
}
