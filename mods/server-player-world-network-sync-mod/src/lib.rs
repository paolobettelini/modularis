use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_registry_api::{ServerPlayerRegistryApi, ServerPlayerSessionSet};
use server_player_world_api::{
    ServerPlayerWorldApi, ServerPlayerWorldChanged, ServerPlayerWorldSet,
};
use tokio::task::JoinHandle;
use world_context_network_message_types::PlayerWorldChanged;

pub struct ServerPlayerWorldNetworkSyncMod;

impl ServerPlayerWorldNetworkSyncMod {
    pub fn init<W: ServerPlayerWorldApi, N: ServerNetworkEventsApi, P: ServerPlayerRegistryApi>(
        bevy: &mut BevyMod,
        _worlds: &mut W,
        _network: &mut N,
        _players: &mut P,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            sync_world_changes
                .in_set(ServerPlayerWorldSet::Sync)
                .after(ServerPlayerSessionSet::Sync),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn sync_world_changes(
    mut changes: MessageReader<ServerPlayerWorldChanged>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for change in changes.read() {
        packets.write(ServerPacketOut {
            audience: ServerAudience::Player(change.player_id),
            message: ClientBoundMessage::PlayerWorldChanged(PlayerWorldChanged {
                world_id: change.current.to_string(),
                position: change.position,
            }),
        });
    }
}
