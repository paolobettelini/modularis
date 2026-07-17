use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use player_gravity_network_message_types::PlayerGravityChanged;
use player_network_message_types::{NetworkPlayer, PlayerId};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_gravity_api::{
    ServerPlayerGravities, ServerPlayerGravityApi, ServerPlayerGravityChanged,
    ServerPlayerGravitySet,
};
use server_player_lifecycle_events_api::ServerPlayerJoined;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use server_player_visibility_api::{ServerPlayerVisibility, ServerPlayerVisibilityApi};
use tokio::task::JoinHandle;

pub struct ServerPlayerGravityNetworkSyncMod;

impl ServerPlayerGravityNetworkSyncMod {
    pub fn init<
        N: ServerNetworkEventsApi,
        G: ServerPlayerGravityApi,
        R: ServerPlayerRegistryApi,
        V: ServerPlayerVisibilityApi,
    >(
        bevy: &mut BevyMod,
        _network_events: &mut N,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _gravity: &mut G,
        _players: &mut R,
        _visibility: &mut V,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            (sync_gravity_to_new_players, sync_gravity_changes)
                .in_set(ServerPlayerGravitySet::Sync),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn sync_gravity_to_new_players(
    gravities: Res<ServerPlayerGravities>,
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
                send_gravity(
                    ServerAudience::Player(joining_player.id),
                    subject.id,
                    gravities.gravity(subject.id),
                    &mut packets,
                );
            }
        }

        let viewers = visibility.viewers_of(joining_player, &all_players);
        if !viewers.is_empty() {
            send_gravity(
                ServerAudience::Players(viewers),
                joining_player.id,
                gravities.gravity(joining_player.id),
                &mut packets,
            );
        }
    }
}

fn sync_gravity_changes(
    gravities: Res<ServerPlayerGravities>,
    players: Res<ServerPlayerRegistry>,
    visibility: Res<ServerPlayerVisibility>,
    mut changes: MessageReader<ServerPlayerGravityChanged>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    let all_players = players.players();
    for change in changes.read() {
        let Some(subject) = find_player(&all_players, change.player_id) else {
            continue;
        };
        let mut recipients = visibility.viewers_of(subject, &all_players);
        recipients.push(subject.id);
        recipients.sort_unstable();
        recipients.dedup();
        send_gravity(
            ServerAudience::Players(recipients),
            subject.id,
            gravities.gravity(subject.id),
            &mut packets,
        );
    }
}

fn find_player(players: &[NetworkPlayer], player_id: PlayerId) -> Option<&NetworkPlayer> {
    players.iter().find(|player| player.id == player_id)
}

fn send_gravity(
    audience: ServerAudience,
    player_id: PlayerId,
    gravity: Vec3,
    packets: &mut MessageWriter<ServerPacketOut>,
) {
    packets.write(ServerPacketOut {
        audience,
        message: ClientBoundMessage::PlayerGravityChanged(PlayerGravityChanged {
            player_id,
            gravity: gravity.to_array(),
        }),
    });
}
