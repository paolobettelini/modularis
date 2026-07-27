use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use player_network_message_types::{NetworkPlayer, PlayerId, PlayerJoined, PlayerLeft};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use server_scope_api::{
    ScopeFacetId, ScopeNodeId, ServerPlayerScopeChanged, ServerScopeApi, ServerScopeSet,
    ServerScopes,
};
use std::collections::HashSet;
use tokio::task::JoinHandle;

pub struct ServerPlayerVisibilityScopeSyncMod;

impl ServerPlayerVisibilityScopeSyncMod {
    pub fn init<S: ServerScopeApi, P: ServerPlayerRegistryApi, N: ServerNetworkEventsApi>(
        bevy: &mut BevyMod,
        _scopes_api: &mut S,
        _players_api: &mut P,
        _network: &mut N,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            sync_visibility_changes.in_set(ServerScopeSet::React),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn sync_visibility_changes(
    scopes: Res<ServerScopes>,
    players: Res<ServerPlayerRegistry>,
    mut changes: MessageReader<ServerPlayerScopeChanged>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    let snapshot = players.players();
    for change in changes.read() {
        // Session join/leave already owns initial snapshots and disconnect
        // packets. This system only reconciles a live migration.
        if change.previous.is_none() || change.current.is_none() {
            continue;
        }
        let Some(subject) = players.player(change.player_id).cloned() else {
            continue;
        };
        let old_boundary = change
            .previous
            .as_ref()
            .and_then(|scope| scopes.resolve_facet(scope, &ScopeFacetId::visibility()));
        let new_boundary = change
            .current
            .as_ref()
            .and_then(|scope| scopes.resolve_facet(scope, &ScopeFacetId::visibility()));
        if old_boundary == new_boundary {
            continue;
        }

        let old_visible = visible_players_at(&scopes, &snapshot, subject.id, old_boundary.as_ref());
        let new_visible = visible_players_at(&scopes, &snapshot, subject.id, new_boundary.as_ref());
        let leaving = old_visible
            .difference(&new_visible)
            .copied()
            .collect::<Vec<_>>();
        let entering = new_visible
            .difference(&old_visible)
            .copied()
            .collect::<Vec<_>>();

        for player_id in &leaving {
            packets.write(ServerPacketOut {
                audience: ServerAudience::Player(subject.id),
                message: ClientBoundMessage::PlayerLeft(PlayerLeft {
                    player_id: *player_id,
                }),
            });
        }
        for player_id in &entering {
            if let Some(player) = players.player(*player_id).cloned() {
                packets.write(ServerPacketOut {
                    audience: ServerAudience::Player(subject.id),
                    message: ClientBoundMessage::PlayerJoined(PlayerJoined { player }),
                });
            }
        }
        if !leaving.is_empty() {
            packets.write(ServerPacketOut {
                audience: ServerAudience::Players(leaving),
                message: ClientBoundMessage::PlayerLeft(PlayerLeft {
                    player_id: subject.id,
                }),
            });
        }
        if !entering.is_empty() {
            packets.write(ServerPacketOut {
                audience: ServerAudience::Players(entering),
                message: ClientBoundMessage::PlayerJoined(PlayerJoined { player: subject }),
            });
        }
    }
}

fn visible_players_at(
    scopes: &ServerScopes,
    players: &[NetworkPlayer],
    subject: PlayerId,
    boundary: Option<&ScopeNodeId>,
) -> HashSet<PlayerId> {
    let Some(boundary) = boundary else {
        return HashSet::new();
    };
    players
        .iter()
        .filter(|player| player.id != subject)
        .filter(|player| {
            scopes
                .resolve_player_facet(player.id, &ScopeFacetId::visibility())
                .as_ref()
                == Some(boundary)
        })
        .map(|player| player.id)
        .collect()
}
