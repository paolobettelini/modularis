use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_player_gravity_map_api::{
    ClientPlayerGravities, ClientPlayerGravityChanged, ClientPlayerGravityMapApi,
    ClientPlayerGravityMapSet,
};
use client_session_api::{ClientSession, ClientSessionApi};
use generated_network_messages::{
    NetworkMessageSet, PlayerGravityChangedReceived, PlayerLeftReceived,
};
use network_protocol_mod::NetworkProtocolMod;
use player_gravity_api::{Gravity, PlayerGravityApi};
use tokio::task::JoinHandle;

pub struct ClientPlayerGravityNetworkReceiveMod;

impl ClientPlayerGravityNetworkReceiveMod {
    pub fn init<G: PlayerGravityApi, M: ClientPlayerGravityMapApi, S: ClientSessionApi>(
        bevy: &mut BevyMod,
        _gravity: &mut G,
        _gravity_map: &mut M,
        _session: &mut S,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            (apply_server_gravity, remove_left_player_gravity)
                .chain()
                .in_set(ClientPlayerGravityMapSet)
                .after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_server_gravity(
    mut messages: MessageReader<PlayerGravityChangedReceived>,
    session: Res<ClientSession>,
    mut local_gravity: ResMut<Gravity>,
    mut gravities: ResMut<ClientPlayerGravities>,
    mut changed: MessageWriter<ClientPlayerGravityChanged>,
) {
    for message in messages.read() {
        let packet = &message.0;
        let gravity = Vec3::from_array(packet.gravity);
        if !gravity.is_finite() {
            continue;
        }
        if session.player_id == Some(packet.player_id) {
            local_gravity.0 = gravity;
        }
        if gravities.set(packet.player_id, gravity) {
            changed.write(ClientPlayerGravityChanged {
                player_id: packet.player_id,
                gravity,
            });
        }
    }
}

fn remove_left_player_gravity(
    mut left: MessageReader<PlayerLeftReceived>,
    mut gravities: ResMut<ClientPlayerGravities>,
) {
    for left in left.read() {
        gravities.remove(left.0.player_id);
    }
}
