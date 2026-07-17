use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_player_scale_map_api::{
    ClientPlayerScaleChanged, ClientPlayerScaleMapApi, ClientPlayerScaleMapSet, ClientPlayerScales,
};
use client_session_api::{ClientSession, ClientSessionApi};
use generated_network_messages::{
    NetworkMessageSet, PlayerLeftReceived, PlayerScaleChangedReceived,
};
use network_protocol_mod::NetworkProtocolMod;
use player_scale_api::{PlayerScale, PlayerScaleApi};
use tokio::task::JoinHandle;

pub struct ClientPlayerScaleNetworkReceiveMod;

impl ClientPlayerScaleNetworkReceiveMod {
    pub fn init<S: PlayerScaleApi, M: ClientPlayerScaleMapApi, C: ClientSessionApi>(
        bevy: &mut BevyMod,
        _scale: &mut S,
        _scale_map: &mut M,
        _session: &mut C,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            (apply_server_scale, remove_left_player_scale)
                .chain()
                .in_set(ClientPlayerScaleMapSet)
                .after(NetworkMessageSet::DispatchPackets),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_server_scale(
    mut messages: MessageReader<PlayerScaleChangedReceived>,
    session: Res<ClientSession>,
    mut local_scale: ResMut<PlayerScale>,
    mut scales: ResMut<ClientPlayerScales>,
    mut changed: MessageWriter<ClientPlayerScaleChanged>,
) {
    for message in messages.read() {
        let packet = &message.0;
        if !packet.scale.is_finite() || packet.scale <= 0.0 {
            continue;
        }
        if session.player_id == Some(packet.player_id) {
            local_scale.0 = packet.scale;
        }
        if scales.set(packet.player_id, packet.scale) {
            changed.write(ClientPlayerScaleChanged {
                player_id: packet.player_id,
                scale: packet.scale,
            });
        }
    }
}

fn remove_left_player_scale(
    mut left: MessageReader<PlayerLeftReceived>,
    mut scales: ResMut<ClientPlayerScales>,
) {
    for left in left.read() {
        scales.remove(left.0.player_id);
    }
}
