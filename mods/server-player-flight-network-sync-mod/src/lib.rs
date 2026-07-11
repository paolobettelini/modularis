use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use player_flight_network_message_types::PlayerFlightCapabilityChanged;
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_flight_api::{
    ServerPlayerFlightApi, ServerPlayerFlightCapabilityChanged, ServerPlayerFlightSet,
};
use tokio::task::JoinHandle;

pub struct ServerPlayerFlightNetworkSyncMod;

impl ServerPlayerFlightNetworkSyncMod {
    pub fn init<F: ServerPlayerFlightApi, N: ServerNetworkEventsApi>(
        bevy: &mut BevyMod,
        _flight: &mut F,
        _network: &mut N,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            sync_capability_changes.in_set(ServerPlayerFlightSet::Sync),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn sync_capability_changes(
    mut changes: MessageReader<ServerPlayerFlightCapabilityChanged>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for change in changes.read() {
        packets.write(ServerPacketOut {
            audience: ServerAudience::Player(change.player_id),
            message: ClientBoundMessage::PlayerFlightCapabilityChanged(
                PlayerFlightCapabilityChanged {
                    enabled: change.enabled,
                },
            ),
        });
    }
}
