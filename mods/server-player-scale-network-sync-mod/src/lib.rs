use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use player_network_message_types::{NetworkPlayer, PlayerId};
use player_scale_network_message_types::PlayerScaleChanged;
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_lifecycle_events_api::ServerPlayerJoined;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::{
    ServerPlayerRegistry, ServerPlayerRegistryApi, ServerPlayerSessionSet,
};
use server_player_scale_api::{
    ServerPlayerScaleApi, ServerPlayerScaleChanged, ServerPlayerScaleSet, ServerPlayerScales,
};
use server_player_visibility_api::{ServerPlayerVisibility, ServerPlayerVisibilityApi};
use tokio::task::JoinHandle;

pub struct ServerPlayerScaleNetworkSyncMod;

impl ServerPlayerScaleNetworkSyncMod {
    pub fn init<
        S: ServerPlayerScaleApi,
        N: ServerNetworkEventsApi,
        R: ServerPlayerRegistryApi,
        V: ServerPlayerVisibilityApi,
    >(
        bevy: &mut BevyMod,
        _scale: &mut S,
        _network: &mut N,
        _players: &mut R,
        _visibility: &mut V,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            (sync_joined_player_state, sync_scale_changes)
                .in_set(ServerPlayerScaleSet::Sync)
                .after(ServerPlayerSessionSet::Sync),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn sync_joined_player_state(
    scales: Res<ServerPlayerScales>,
    players: Res<ServerPlayerRegistry>,
    visibility: Res<ServerPlayerVisibility>,
    mut joined: MessageReader<ServerPlayerJoined>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    let all_players = players.players();
    for joined in joined.read() {
        let Some(joining_player) = find_player(&all_players, joined.player_id) else {
            continue;
        };

        for subject in &all_players {
            if subject.id == joining_player.id || visibility.can_see(joining_player, subject) {
                send_scale(
                    ServerAudience::Player(joining_player.id),
                    subject.id,
                    scales.scale(subject.id),
                    &mut packets,
                );
            }
        }

        let viewers = visibility.viewers_of(joining_player, &all_players);
        if !viewers.is_empty() {
            send_scale(
                ServerAudience::Players(viewers),
                joining_player.id,
                scales.scale(joining_player.id),
                &mut packets,
            );
        }
    }
}

fn sync_scale_changes(
    scales: Res<ServerPlayerScales>,
    players: Res<ServerPlayerRegistry>,
    visibility: Res<ServerPlayerVisibility>,
    mut changed: MessageReader<ServerPlayerScaleChanged>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    let all_players = players.players();
    for change in changed.read() {
        let Some(subject) = find_player(&all_players, change.player_id) else {
            continue;
        };
        let mut recipients = visibility.viewers_of(subject, &all_players);
        recipients.push(subject.id);
        recipients.sort_unstable();
        recipients.dedup();
        send_scale(
            ServerAudience::Players(recipients),
            subject.id,
            scales.scale(subject.id),
            &mut packets,
        );
    }
}

fn find_player(players: &[NetworkPlayer], player_id: PlayerId) -> Option<&NetworkPlayer> {
    players.iter().find(|player| player.id == player_id)
}

fn send_scale(
    audience: ServerAudience,
    player_id: PlayerId,
    scale: f32,
    packets: &mut MessageWriter<ServerPacketOut>,
) {
    packets.write(ServerPacketOut {
        audience,
        message: ClientBoundMessage::PlayerScaleChanged(PlayerScaleChanged { player_id, scale }),
    });
}
