use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use player_flight_network_message_types::PlayerFlightCapabilityChanged;
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_flight_api::{
    ServerPlayerFlightApi, ServerPlayerFlightCapabilities, ServerPlayerFlightCapabilityChanged,
    ServerPlayerFlightSet,
};
use server_player_lifecycle_events_api::ServerPlayerJoined;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::ServerPlayerSessionSet;
use tokio::task::JoinHandle;

pub struct ServerPlayerFlightNetworkSyncMod;

impl ServerPlayerFlightNetworkSyncMod {
    pub fn init<F: ServerPlayerFlightApi, N: ServerNetworkEventsApi>(
        bevy: &mut BevyMod,
        _flight: &mut F,
        _network: &mut N,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            (sync_joined_players, sync_capability_changes)
                .in_set(ServerPlayerFlightSet::Sync)
                .after(ServerPlayerSessionSet::Initialize),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn sync_joined_players(
    capabilities: Res<ServerPlayerFlightCapabilities>,
    mut joined: MessageReader<ServerPlayerJoined>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for player in joined.read() {
        send_capability(
            player.player_id,
            capabilities.enabled(player.player_id),
            &mut packets,
        );
    }
}

fn sync_capability_changes(
    mut changes: MessageReader<ServerPlayerFlightCapabilityChanged>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for change in changes.read() {
        send_capability(change.player_id, change.enabled, &mut packets);
    }
}

fn send_capability(
    player_id: player_network_message_types::PlayerId,
    enabled: bool,
    packets: &mut MessageWriter<ServerPacketOut>,
) {
    packets.write(ServerPacketOut {
        audience: ServerAudience::Player(player_id),
        message: ClientBoundMessage::PlayerFlightCapabilityChanged(
            PlayerFlightCapabilityChanged { enabled },
        ),
    });
}
