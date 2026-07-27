use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_lifecycle_events_api::ServerPlayerJoined;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::ServerPlayerSessionSet;
use server_sun_api::{ServerSunApi, ServerSunChanged, ServerSunSet, ServerSunState};
use sun_network_message_types::SunSettingsChanged;
use tokio::task::JoinHandle;

pub struct ServerSunNetworkSyncMod;

impl ServerSunNetworkSyncMod {
    pub fn init<S: ServerSunApi, N: ServerNetworkEventsApi>(
        bevy: &mut BevyMod,
        _sun: &mut S,
        _network: &mut N,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            (sync_joining_players, broadcast_sun_changes)
                .in_set(ServerSunSet::Sync)
                .after(ServerPlayerSessionSet::Sync),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn sync_joining_players(
    state: Res<ServerSunState>,
    mut joined: MessageReader<ServerPlayerJoined>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    let Some(settings) = state.current else {
        return;
    };
    for player in joined.read() {
        packets.write(ServerPacketOut {
            audience: ServerAudience::Player(player.player_id),
            message: sun_message(settings),
        });
    }
}

fn broadcast_sun_changes(
    mut changes: MessageReader<ServerSunChanged>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for change in changes.read() {
        packets.write(ServerPacketOut {
            audience: ServerAudience::Broadcast,
            message: sun_message(change.current),
        });
    }
}

fn sun_message(settings: sun_api::SunSettings) -> ClientBoundMessage {
    ClientBoundMessage::SunSettingsChanged(SunSettingsChanged { settings })
}
