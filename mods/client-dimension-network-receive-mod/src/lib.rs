use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_dimension_api::{
    ClientDimension, ClientDimensionApi, ClientDimensionChanged, ClientDimensionSet,
};
use client_session_api::{ClientSession, ClientSessionApi};
use generated_network_messages::{NetworkMessageSet, PlayerDimensionChangedReceived};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientDimensionNetworkReceiveMod;

impl ClientDimensionNetworkReceiveMod {
    pub fn init<D: ClientDimensionApi, S: ClientSessionApi>(
        bevy: &mut BevyMod,
        _dimension: &mut D,
        _session: &mut S,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_dimension_changes
                .after(NetworkMessageSet::DispatchPackets)
                .in_set(ClientDimensionSet::Receive),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_dimension_changes(
    mut packets: MessageReader<PlayerDimensionChangedReceived>,
    mut session: ResMut<ClientSession>,
    mut dimension: ResMut<ClientDimension>,
    mut changes: MessageWriter<ClientDimensionChanged>,
) {
    for packet in packets.read() {
        if !session.accept_movement_epoch(packet.0.movement_epoch) {
            continue;
        }
        let previous = std::mem::replace(&mut dimension.0, packet.0.dimension);
        changes.write(ClientDimensionChanged {
            previous,
            current: packet.0.dimension,
            position: packet.0.position,
        });
    }
}
