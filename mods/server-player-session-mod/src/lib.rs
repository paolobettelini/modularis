use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::{
    ClientBoundMessage, JoinRequestReceived, LeaveRequestReceived, NetworkMessageSet,
    PlayerMoveReceived,
};
use network_protocol_mod::NetworkProtocolMod;
use player_network_message_types::{PlayerJoined, PlayerLeft, PlayerMoved, PlayerRotationChanged};
use server_network_api::{ServerNetworkApi, ServerNetworkSender};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_lifecycle_events_api::{ServerPlayerJoined, ServerPlayerLeft};
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::{
    PendingServerPlayerMove, PendingServerPlayerMoves, ServerPlayerMovementSet,
    ServerPlayerRegistry, ServerPlayerRegistryApi,
};
use server_player_visibility_api::{ServerPlayerVisibility, ServerPlayerVisibilityApi};
use session_network_message_types::JoinAccepted;
use tokio::task::JoinHandle;

const LOCAL_PLAYER_CORRECTION_THRESHOLD: f32 = 0.15;

pub struct ServerPlayerSessionMod;

impl ServerPlayerSessionMod {
    pub fn init<N: ServerNetworkApi, E: ServerNetworkEventsApi, V: ServerPlayerVisibilityApi>(
        bevy: &mut BevyMod,
        _network: &mut N,
        _network_events: &mut E,
        _protocol: &mut NetworkProtocolMod,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _visibility: &mut V,
    ) -> Self {
        bevy.app
            .init_resource::<ServerPlayerRegistry>()
            .init_resource::<PendingServerPlayerMoves>()
            .configure_sets(
                Update,
                (
                    ServerPlayerMovementSet::Receive,
                    ServerPlayerMovementSet::Validate,
                    ServerPlayerMovementSet::Apply,
                    ServerPlayerMovementSet::Sync,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (handle_join, handle_leave).after(NetworkMessageSet::DispatchPackets),
            )
            .add_systems(
                Update,
                collect_movement_requests
                    .after(NetworkMessageSet::DispatchPackets)
                    .in_set(ServerPlayerMovementSet::Receive),
            )
            .add_systems(
                Update,
                apply_validated_movements.in_set(ServerPlayerMovementSet::Apply),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerPlayerRegistryApi for ServerPlayerSessionMod {}

fn handle_join(
    mut joins: MessageReader<JoinRequestReceived>,
    mut registry: ResMut<ServerPlayerRegistry>,
    network: Res<ServerNetworkSender>,
    time: Res<Time>,
    mut joined: MessageWriter<ServerPlayerJoined>,
    mut packets: MessageWriter<ServerPacketOut>,
    visibility: Res<ServerPlayerVisibility>,
) {
    for join in joins.read() {
        let name = {
            let trimmed = join.message.name.trim();
            if trimmed.is_empty() {
                "Player".to_string()
            } else {
                trimmed.chars().take(32).collect()
            }
        };
        let already_joined = registry.player_for_address(join.source).is_some();
        let player = registry.join(join.source, name, time.elapsed_secs_f64());
        network.register_client(join.source);
        let visible_players = registry
            .players()
            .into_iter()
            .filter(|candidate| visibility.can_see(&player, candidate))
            .collect();
        let accepted = ClientBoundMessage::JoinAccepted(JoinAccepted {
            player_id: player.id,
            players: visible_players,
        });
        packets.write(ServerPacketOut {
            audience: ServerAudience::Address(join.source),
            message: accepted,
        });
        if !already_joined {
            joined.write(ServerPlayerJoined {
                player_id: player.id,
            });
            let viewers = visibility.viewers_of(&player, &registry.players());
            packets.write(ServerPacketOut {
                audience: ServerAudience::Players(viewers),
                message: ClientBoundMessage::PlayerJoined(PlayerJoined { player }),
            });
        }
    }
}

fn handle_leave(
    mut leaves: MessageReader<LeaveRequestReceived>,
    mut registry: ResMut<ServerPlayerRegistry>,
    network: Res<ServerNetworkSender>,
    mut left: MessageWriter<ServerPlayerLeft>,
    mut packets: MessageWriter<ServerPacketOut>,
    visibility: Res<ServerPlayerVisibility>,
) {
    for leave in leaves.read() {
        let viewers = registry
            .player_for_address(leave.source)
            .map(|player| visibility.viewers_of(player, &registry.players()))
            .unwrap_or_default();
        if let Some(player) = registry.leave(leave.source) {
            left.write(ServerPlayerLeft {
                player_id: player.id,
            });
            network.remove_client(leave.source);
            packets.write(ServerPacketOut {
                audience: ServerAudience::Players(viewers),
                message: ClientBoundMessage::PlayerLeft(PlayerLeft {
                    player_id: player.id,
                }),
            });
        }
    }
}

fn collect_movement_requests(
    mut movements: MessageReader<PlayerMoveReceived>,
    mut registry: ResMut<ServerPlayerRegistry>,
    time: Res<Time>,
    mut pending: ResMut<PendingServerPlayerMoves>,
) {
    for movement in movements.read() {
        let Some(player) = registry.player_for_address(movement.source) else {
            continue;
        };
        let current_position = Vec3::from_array(player.position);
        let requested_position = Vec3::from_array(movement.message.position);
        let player_id = player.id;
        registry.touch_address(movement.source, time.elapsed_secs_f64());
        pending.moves.push(PendingServerPlayerMove {
            source: movement.source,
            player_id,
            current_position,
            requested_position,
            accepted_position: requested_position,
            yaw: movement.message.yaw,
            pitch: movement.message.pitch,
            rejected: false,
        });
    }
}

fn apply_validated_movements(
    mut registry: ResMut<ServerPlayerRegistry>,
    time: Res<Time>,
    mut pending: ResMut<PendingServerPlayerMoves>,
    mut packets: MessageWriter<ServerPacketOut>,
    visibility: Res<ServerPlayerVisibility>,
) {
    let moves = std::mem::take(&mut pending.moves);
    for movement in moves {
        if let Some(player) = registry.apply_player_move(
            movement.source,
            movement.player_id,
            if movement.rejected {
                movement.current_position
            } else {
                movement.accepted_position
            },
            movement.yaw,
            movement.pitch,
            time.elapsed_secs_f64(),
        ) {
            let moved = ClientBoundMessage::PlayerMoved(PlayerMoved {
                player_id: player.id,
                position: player.position,
                yaw: player.yaw,
                pitch: player.pitch,
            });
            let viewers = visibility.viewers_of(&player, &registry.players());
            packets.write(ServerPacketOut {
                audience: ServerAudience::Players(viewers.clone()),
                message: moved.clone(),
            });
            packets.write(ServerPacketOut {
                audience: ServerAudience::Players(viewers),
                message: ClientBoundMessage::PlayerRotationChanged(PlayerRotationChanged {
                    player_id: player.id,
                    yaw: player.yaw,
                    pitch: player.pitch,
                }),
            });
            if movement.rejected
                || should_correct_local_player(movement.requested_position, &player)
            {
                packets.write(ServerPacketOut {
                    audience: ServerAudience::Address(movement.source),
                    message: moved,
                });
            }
        }
    }
}

fn should_correct_local_player(
    requested_position: Vec3,
    accepted_player: &player_network_message_types::NetworkPlayer,
) -> bool {
    let accepted_position = Vec3::from_array(accepted_player.position);
    requested_position.distance(accepted_position) > LOCAL_PLAYER_CORRECTION_THRESHOLD
}
