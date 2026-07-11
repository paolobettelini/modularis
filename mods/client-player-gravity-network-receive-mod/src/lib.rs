use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::{NetworkMessageSet, PlayerGravityChangedReceived};
use network_protocol_mod::NetworkProtocolMod;
use player_gravity_api::{Gravity, PlayerGravityApi};
use tokio::task::JoinHandle;

pub struct ClientPlayerGravityNetworkReceiveMod;

impl ClientPlayerGravityNetworkReceiveMod {
    pub fn init<G: PlayerGravityApi>(
        bevy: &mut BevyMod,
        _gravity: &mut G,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            apply_server_gravity.after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_server_gravity(
    mut messages: MessageReader<PlayerGravityChangedReceived>,
    mut gravity: ResMut<Gravity>,
) {
    for message in messages.read() {
        gravity.0 = Vec3::from_array(message.0.gravity);
    }
}
