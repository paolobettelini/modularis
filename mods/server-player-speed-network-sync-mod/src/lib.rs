use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use player_speed_network_message_types::PlayerSpeedChanged;
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_lifecycle_events_api::ServerPlayerJoined;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::ServerPlayerSessionSet;
use server_player_speed_api::{
    ServerPlayerSpeedApi, ServerPlayerSpeedChanged, ServerPlayerSpeedSet, ServerPlayerSpeeds,
};
use tokio::task::JoinHandle;

pub struct ServerPlayerSpeedNetworkSyncMod;

impl ServerPlayerSpeedNetworkSyncMod {
    pub fn init<S: ServerPlayerSpeedApi, N: ServerNetworkEventsApi>(
        bevy: &mut BevyMod,
        _speed: &mut S,
        _network: &mut N,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            (sync_joined_players, sync_speed_changes)
                .in_set(ServerPlayerSpeedSet::Sync)
                .after(ServerPlayerSessionSet::Sync),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn sync_joined_players(
    speeds: Res<ServerPlayerSpeeds>,
    mut joined: MessageReader<ServerPlayerJoined>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for player in joined.read() {
        send_speed(
            player.player_id,
            speeds.multiplier(player.player_id),
            &mut packets,
        );
    }
}

fn sync_speed_changes(
    mut changed: MessageReader<ServerPlayerSpeedChanged>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for change in changed.read() {
        send_speed(change.player_id, change.multiplier, &mut packets);
    }
}

fn send_speed(
    player_id: player_network_message_types::PlayerId,
    multiplier: f32,
    packets: &mut MessageWriter<ServerPacketOut>,
) {
    packets.write(ServerPacketOut {
        audience: ServerAudience::Player(player_id),
        message: ClientBoundMessage::PlayerSpeedChanged(PlayerSpeedChanged { multiplier }),
    });
}
