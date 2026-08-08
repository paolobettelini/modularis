use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::{
    ClientBoundMessage, JoinRequestReceived, LeaveRequestReceived, NetworkMessageSet,
    PlayerMoveReceived,
};
use network_protocol_mod::NetworkProtocolMod;
use player_network_message_types::{PlayerJoined, PlayerLeft, PlayerMoved, PlayerRotationChanged};
use server_kick_api::{ServerKickApi, ServerKickRequested, ServerKickSet, ServerKickTarget};
use server_network_api::{ServerNetworkApi, ServerNetworkSender};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_admission_api::{ServerJoinCandidate, ServerPlayerAdmissionRules};
use server_player_lifecycle_events_api::{ServerPlayerJoined, ServerPlayerLeft, ServerPlayerReady};
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::{
    PendingServerPlayerMove, PendingServerPlayerMoves, ServerPlayerMovementApplied,
    ServerPlayerMovementSet, ServerPlayerRegistry, ServerPlayerRegistryApi, ServerPlayerSessionSet,
};
use server_player_visibility_api::{ServerPlayerVisibility, ServerPlayerVisibilityApi};
use session_network_message_types::JoinAccepted;
use tokio::task::JoinHandle;

const LOCAL_PLAYER_CORRECTION_THRESHOLD: f32 = 0.15;

#[derive(Debug)]
struct PendingServerJoin {
    source: std::net::SocketAddr,
    name: String,
    rejection: Option<String>,
    player: Option<player_network_message_types::NetworkPlayer>,
    newly_joined: bool,
}

#[derive(Resource, Default)]
struct PendingServerJoins(Vec<PendingServerJoin>);

#[derive(Resource, Default)]
struct AnonymousPlayerNameSequence(u64);

pub struct ServerPlayerSessionMod;

impl ServerPlayerSessionMod {
    pub fn init<
        N: ServerNetworkApi,
        E: ServerNetworkEventsApi,
        V: ServerPlayerVisibilityApi,
        K: ServerKickApi,
    >(
        bevy: &mut BevyMod,
        _network: &mut N,
        _network_events: &mut E,
        _protocol: &mut NetworkProtocolMod,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _visibility: &mut V,
        _kick: &mut K,
    ) -> Self {
        bevy.app
            .init_resource::<ServerPlayerRegistry>()
            .init_resource::<ServerPlayerAdmissionRules>()
            .init_resource::<PendingServerPlayerMoves>()
            .init_resource::<PendingServerJoins>()
            .init_resource::<AnonymousPlayerNameSequence>()
            .add_message::<ServerPlayerMovementApplied>()
            .configure_sets(
                Update,
                (
                    ServerPlayerSessionSet::Receive,
                    ServerPlayerSessionSet::Validate,
                    ServerPlayerSessionSet::Register,
                    ServerPlayerSessionSet::Initialize,
                    ServerPlayerSessionSet::Sync,
                    ServerPlayerSessionSet::Cleanup,
                )
                    .chain(),
            )
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
                (
                    collect_join_requests
                        .after(NetworkMessageSet::DispatchPackets)
                        .in_set(ServerPlayerSessionSet::Receive),
                    validate_join_requests
                        .in_set(ServerPlayerSessionSet::Validate)
                        .before(ServerKickSet::Apply),
                    register_joined_players.in_set(ServerPlayerSessionSet::Register),
                    sync_joined_players.in_set(ServerPlayerSessionSet::Sync),
                    handle_leave
                        .after(NetworkMessageSet::DispatchPackets)
                        .in_set(ServerPlayerSessionSet::Cleanup),
                ),
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

fn collect_join_requests(
    mut joins: MessageReader<JoinRequestReceived>,
    mut pending: ResMut<PendingServerJoins>,
    mut anonymous_names: ResMut<AnonymousPlayerNameSequence>,
) {
    for join in joins.read() {
        anonymous_names.0 = anonymous_names.0.wrapping_add(1);
        pending.0.push(PendingServerJoin {
            source: join.source,
            name: format!("Player{}", anonymous_names.0),
            rejection: None,
            player: None,
            newly_joined: false,
        });
    }
}

fn validate_join_requests(
    registry: Res<ServerPlayerRegistry>,
    admission: Res<ServerPlayerAdmissionRules>,
    mut pending: ResMut<PendingServerJoins>,
) {
    for join in &mut pending.0 {
        if registry.player_for_address(join.source).is_none() {
            let mut candidate = ServerJoinCandidate {
                address: join.source,
                name: join.name.clone(),
            };
            match admission.validate(&mut candidate, &registry.players()) {
                Ok(()) => join.name = candidate.name,
                Err(reason) => join.rejection = Some(reason),
            }
        }
    }
}

fn register_joined_players(
    mut registry: ResMut<ServerPlayerRegistry>,
    network: Res<ServerNetworkSender>,
    time: Res<Time>,
    mut pending: ResMut<PendingServerJoins>,
    mut joined: MessageWriter<ServerPlayerJoined>,
    mut kicks: MessageWriter<ServerKickRequested>,
) {
    for join in &mut pending.0 {
        if let Some(reason) = join.rejection.clone() {
            kicks.write(ServerKickRequested {
                target: ServerKickTarget::Address(join.source),
                reason,
            });
            continue;
        }
        join.newly_joined = registry.player_for_address(join.source).is_none();
        let player = registry.join(join.source, join.name.clone(), time.elapsed_secs_f64());
        network.register_client(join.source);
        if join.newly_joined {
            joined.write(ServerPlayerJoined {
                player_id: player.id,
            });
        }
        join.player = Some(player);
    }
}

fn sync_joined_players(
    registry: Res<ServerPlayerRegistry>,
    visibility: Res<ServerPlayerVisibility>,
    mut pending: ResMut<PendingServerJoins>,
    mut packets: MessageWriter<ServerPacketOut>,
    mut ready: MessageWriter<ServerPlayerReady>,
) {
    for join in std::mem::take(&mut pending.0) {
        let Some(player) = join.player else {
            continue;
        };
        let player_id = player.id;
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
        if join.newly_joined {
            let viewers = visibility.viewers_of(&player, &registry.players());
            packets.write(ServerPacketOut {
                audience: ServerAudience::Players(viewers),
                message: ClientBoundMessage::PlayerJoined(PlayerJoined { player }),
            });
        }
        ready.write(ServerPlayerReady { player_id });
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
    mut applied: MessageWriter<ServerPlayerMovementApplied>,
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
            let position = Vec3::from_array(player.position);
            applied.write(ServerPlayerMovementApplied {
                player_id: player.id,
                previous_position: movement.current_position,
                position,
                yaw: player.yaw,
                pitch: player.pitch,
                corrected: movement.rejected
                    || movement.requested_position.distance(position)
                        > LOCAL_PLAYER_CORRECTION_THRESHOLD,
            });
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
