use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_world_context_api::{
    ClientWorldChanged, ClientWorldContext, ClientWorldContextApi, ClientWorldContextSet,
};
use generated_network_messages::{NetworkMessageSet, PlayerWorldChangedReceived};
use network_protocol_mod::NetworkProtocolMod;
use tokio::task::JoinHandle;

pub struct ClientWorldContextNetworkReceiveMod;

impl ClientWorldContextNetworkReceiveMod {
    pub fn init<W: ClientWorldContextApi>(
        bevy: &mut BevyMod,
        _world: &mut W,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            receive_world_changes
                .after(NetworkMessageSet::DispatchPackets)
                .in_set(ClientWorldContextSet::Receive),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn receive_world_changes(
    mut packets: MessageReader<PlayerWorldChangedReceived>,
    mut context: ResMut<ClientWorldContext>,
    mut changed: MessageWriter<ClientWorldChanged>,
) {
    for packet in packets.read() {
        if let Some(change) =
            context.apply_authoritative_update(packet.0.world_id.clone(), packet.0.position)
        {
            changed.write(change);
        }
    }
}
