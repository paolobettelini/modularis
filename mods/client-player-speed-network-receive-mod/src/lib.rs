use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::{NetworkMessageSet, PlayerSpeedChangedReceived};
use network_protocol_mod::NetworkProtocolMod;
use player_speed_api::{PlayerSpeedApi, PlayerSpeedMultiplier};
use tokio::task::JoinHandle;

pub struct ClientPlayerSpeedNetworkReceiveMod;

impl ClientPlayerSpeedNetworkReceiveMod {
    pub fn init<S: PlayerSpeedApi>(
        bevy: &mut BevyMod,
        _speed: &mut S,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            apply_server_speed.after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_server_speed(
    mut messages: MessageReader<PlayerSpeedChangedReceived>,
    mut speed: ResMut<PlayerSpeedMultiplier>,
) {
    for message in messages.read() {
        if message.0.multiplier.is_finite() {
            speed.0 = message.0.multiplier.max(0.0);
        }
    }
}
