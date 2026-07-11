use bevy::prelude::*;
use bevy_mod::BevyMod;
use blocky_animation_api::BlockyAnimationApi;
use blocky_model_api::{
    BlockyAnimationPlayback, BlockyAnimationTranslationMask, BlockyModelApi, BlockyModelNode,
    BlockyModelRoot, BlockyModelSpawned, PlayBlockyAnimation, SpawnBlockyModel,
};
use client_camera_api::{CameraApi, PlayerCamera};
use client_game_state_api::{GameState, GameStateApi};
use client_player_blocky_model_paths_api::{
    ClientPlayerBlockyModelPaths, ClientPlayerBlockyModelPathsApi,
};
use client_player_render_api::{
    ClientPlayerRenderApi, NetworkPlayerVisual, RenderedNetworkPlayers,
};
use generated_network_messages::{
    JoinAcceptedReceived, NetworkMessageSet, PlayerJoinedReceived, PlayerLeftReceived,
    PlayerMovedReceived, PlayerRotationChangedReceived,
};
use network_protocol_mod::NetworkProtocolMod;
use player_gravity_api::{Gravity, PlayerGravityApi};
use player_network_message_types::{NetworkPlayer, PlayerId};
use std::collections::HashMap;
use tokio::task::JoinHandle;

const WALK_SPEED_THRESHOLD: f32 = 0.02;
const WALK_TO_IDLE_SECONDS: f64 = 0.20;

pub struct ClientPlayerRenderBlockyImpl;

impl ClientPlayerRenderBlockyImpl {
    pub fn init<
        C: CameraApi,
        V: PlayerGravityApi,
        G: GameStateApi,
        M: BlockyModelApi,
        A: BlockyAnimationApi,
        P: ClientPlayerBlockyModelPathsApi,
    >(
        bevy: &mut BevyMod,
        _camera: &mut C,
        _gravity: &mut V,
        _game_state: &mut G,
        _protocol: &mut NetworkProtocolMod,
        _models: &mut M,
        _animations: &mut A,
        _paths: &mut P,
    ) -> Self {
        bevy.app
            .init_resource::<RenderedNetworkPlayers>()
            .init_resource::<PendingBlockyPlayerSpawns>()
            .add_systems(
                Update,
                (
                    request_initial_players,
                    request_joined_players,
                    attach_spawned_players,
                    move_players,
                    rotate_players,
                    settle_idle_players,
                    project_labels_to_viewport,
                    remove_players,
                    remove_stale_players,
                )
                    .chain()
                    .after(NetworkMessageSet::DispatchPackets)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), clear_rendered_players);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientPlayerRenderApi for ClientPlayerRenderBlockyImpl {}

#[derive(Resource, Default)]
struct PendingBlockyPlayerSpawns {
    next_spawn_id: u64,
    by_spawn_id: HashMap<u64, PendingBlockyPlayerSpawn>,
    by_player_id: HashMap<PlayerId, u64>,
}

#[derive(Debug, Clone)]
struct PendingBlockyPlayerSpawn {
    player: NetworkPlayer,
    last_seen_at: f64,
}

#[derive(Component, Debug, Clone)]
struct BlockyNetworkPlayerAnimation {
    current: NetworkPlayerAnimationKind,
    last_position: Vec3,
    last_motion_at: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NetworkPlayerAnimationKind {
    Idle,
    Walk,
}

fn request_initial_players(
    mut accepted: MessageReader<JoinAcceptedReceived>,
    mut pending: ResMut<PendingBlockyPlayerSpawns>,
    rendered: Res<RenderedNetworkPlayers>,
    paths: Res<ClientPlayerBlockyModelPaths>,
    gravity: Res<Gravity>,
    time: Res<Time>,
    mut spawns: MessageWriter<SpawnBlockyModel>,
) {
    for accepted in accepted.read() {
        for player in &accepted.0.players {
            if player.id == accepted.0.player_id {
                continue;
            }
            request_player_spawn(
                player,
                &mut pending,
                &rendered,
                &paths,
                *gravity,
                time.elapsed_secs_f64(),
                &mut spawns,
            );
        }
    }
}

fn request_joined_players(
    mut joined: MessageReader<PlayerJoinedReceived>,
    mut pending: ResMut<PendingBlockyPlayerSpawns>,
    rendered: Res<RenderedNetworkPlayers>,
    paths: Res<ClientPlayerBlockyModelPaths>,
    gravity: Res<Gravity>,
    time: Res<Time>,
    mut spawns: MessageWriter<SpawnBlockyModel>,
) {
    for joined in joined.read() {
        request_player_spawn(
            &joined.0.player,
            &mut pending,
            &rendered,
            &paths,
            *gravity,
            time.elapsed_secs_f64(),
            &mut spawns,
        );
    }
}

fn request_player_spawn(
    player: &NetworkPlayer,
    pending: &mut PendingBlockyPlayerSpawns,
    rendered: &RenderedNetworkPlayers,
    paths: &ClientPlayerBlockyModelPaths,
    gravity: Gravity,
    now: f64,
    spawns: &mut MessageWriter<SpawnBlockyModel>,
) {
    if rendered.entities.contains_key(&player.id) || pending.by_player_id.contains_key(&player.id) {
        return;
    }
    pending.next_spawn_id = pending.next_spawn_id.wrapping_add(1);
    let spawn_id = pending.next_spawn_id;
    pending.by_player_id.insert(player.id, spawn_id);
    pending.by_spawn_id.insert(
        spawn_id,
        PendingBlockyPlayerSpawn {
            player: player.clone(),
            last_seen_at: now,
        },
    );
    spawns.write(SpawnBlockyModel {
        spawn_id: Some(spawn_id),
        model_path: paths.model_path.to_string(),
        texture_path: paths.texture_path.map(str::to_string),
        texture_size: paths.texture_size,
        transform: Transform {
            translation: Vec3::from_array(player.position),
            rotation: avatar_rotation(gravity, player.yaw, paths.yaw_offset_radians),
            ..default()
        },
        scale: paths.model_scale,
        primitive_scale: paths.primitive_scale,
    });
}

fn attach_spawned_players(
    mut commands: Commands,
    mut spawned: MessageReader<BlockyModelSpawned>,
    mut pending: ResMut<PendingBlockyPlayerSpawns>,
    mut rendered: ResMut<RenderedNetworkPlayers>,
    paths: Res<ClientPlayerBlockyModelPaths>,
    roots: Query<&BlockyModelRoot>,
    nodes: Query<&BlockyModelNode>,
    mut animations: MessageWriter<PlayBlockyAnimation>,
) {
    for spawned in spawned.read() {
        let Some(spawn_id) = spawned.spawn_id else {
            continue;
        };
        let Some(pending_spawn) = pending.by_spawn_id.remove(&spawn_id) else {
            continue;
        };
        pending.by_player_id.remove(&pending_spawn.player.id);
        let position = Vec3::from_array(pending_spawn.player.position);
        commands
            .entity(spawned.root)
            .insert(BlockyNetworkPlayerAnimation {
                current: NetworkPlayerAnimationKind::Idle,
                last_position: position,
                last_motion_at: pending_spawn.last_seen_at,
            });
        lock_configured_vertical_animation_nodes(
            &mut commands,
            spawned.root,
            &paths,
            &roots,
            &nodes,
        );

        let label = commands
            .spawn((
                Text::new(pending_spawn.player.name.clone()),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                UiTransform::from_translation(Val2::percent(-50.0, -100.0)),
                GlobalZIndex(50),
                Visibility::Hidden,
                DespawnOnExit(GameState::InGame),
            ))
            .id();
        rendered.entities.insert(
            pending_spawn.player.id,
            NetworkPlayerVisual {
                avatar: spawned.root,
                label,
                last_seen_at: pending_spawn.last_seen_at,
            },
        );
        animations.write(PlayBlockyAnimation {
            root: spawned.root,
            animation_path: paths.idle_animation_path.to_string(),
            speed: 1.0,
            playback: BlockyAnimationPlayback::Loop,
        });
    }
}

fn lock_configured_vertical_animation_nodes(
    commands: &mut Commands,
    root: Entity,
    paths: &ClientPlayerBlockyModelPaths,
    roots: &Query<&BlockyModelRoot>,
    nodes: &Query<&BlockyModelNode>,
) {
    let Ok(model_root) = roots.get(root) else {
        return;
    };
    for entity in &model_root.node_entities {
        let Ok(node) = nodes.get(*entity) else {
            continue;
        };
        if paths
            .vertical_animation_locked_nodes
            .iter()
            .any(|locked_node| *locked_node == node.name)
        {
            commands
                .entity(*entity)
                .insert(BlockyAnimationTranslationMask {
                    mask: Vec3::new(1.0, 0.0, 1.0),
                });
        }
    }
}

fn move_players(
    mut moved: MessageReader<PlayerMovedReceived>,
    mut pending: ResMut<PendingBlockyPlayerSpawns>,
    mut rendered: ResMut<RenderedNetworkPlayers>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
    gravity: Res<Gravity>,
    paths: Res<ClientPlayerBlockyModelPaths>,
    mut animation_states: Query<&mut BlockyNetworkPlayerAnimation>,
    mut animations: MessageWriter<PlayBlockyAnimation>,
) {
    let now = time.elapsed_secs_f64();
    for moved in moved.read() {
        if let Some(spawn_id) = pending.by_player_id.get(&moved.0.player_id).copied()
            && let Some(pending_spawn) = pending.by_spawn_id.get_mut(&spawn_id)
        {
            pending_spawn.player.position = moved.0.position;
            pending_spawn.player.yaw = moved.0.yaw;
            pending_spawn.player.pitch = moved.0.pitch;
            pending_spawn.last_seen_at = time.elapsed_secs_f64();
        }

        let Some(visual) = rendered.entities.get_mut(&moved.0.player_id) else {
            continue;
        };
        visual.last_seen_at = now;
        let position = Vec3::from_array(moved.0.position);
        if let Ok(mut avatar) = transforms.get_mut(visual.avatar) {
            avatar.translation = position;
            avatar.rotation = avatar_rotation(*gravity, moved.0.yaw, paths.yaw_offset_radians);
        }
        if let Ok(mut state) = animation_states.get_mut(visual.avatar) {
            let delta = position - state.last_position;
            let lateral_delta = delta - gravity.up() * delta.dot(gravity.up());
            state.last_position = position;
            if lateral_delta.length() > WALK_SPEED_THRESHOLD {
                state.last_motion_at = now;
                set_player_animation(
                    visual.avatar,
                    &mut state,
                    NetworkPlayerAnimationKind::Walk,
                    &paths,
                    &mut animations,
                );
            }
        }
    }
}

fn settle_idle_players(
    time: Res<Time>,
    paths: Res<ClientPlayerBlockyModelPaths>,
    rendered: Res<RenderedNetworkPlayers>,
    mut animation_states: Query<&mut BlockyNetworkPlayerAnimation>,
    mut animations: MessageWriter<PlayBlockyAnimation>,
) {
    let now = time.elapsed_secs_f64();
    for visual in rendered.entities.values() {
        let Ok(mut state) = animation_states.get_mut(visual.avatar) else {
            continue;
        };
        if state.current == NetworkPlayerAnimationKind::Walk
            && now - state.last_motion_at >= WALK_TO_IDLE_SECONDS
        {
            set_player_animation(
                visual.avatar,
                &mut state,
                NetworkPlayerAnimationKind::Idle,
                &paths,
                &mut animations,
            );
        }
    }
}

fn set_player_animation(
    root: Entity,
    state: &mut BlockyNetworkPlayerAnimation,
    next: NetworkPlayerAnimationKind,
    paths: &ClientPlayerBlockyModelPaths,
    animations: &mut MessageWriter<PlayBlockyAnimation>,
) {
    if state.current == next {
        return;
    }

    let Some(animation_path) = animation_path(next, paths) else {
        return;
    };

    state.current = next;
    animations.write(PlayBlockyAnimation {
        root,
        animation_path: animation_path.to_string(),
        speed: 1.0,
        playback: match next {
            NetworkPlayerAnimationKind::Idle => BlockyAnimationPlayback::Loop,
            NetworkPlayerAnimationKind::Walk => BlockyAnimationPlayback::Loop,
        },
    });
}

fn animation_path(
    kind: NetworkPlayerAnimationKind,
    paths: &ClientPlayerBlockyModelPaths,
) -> Option<&'static str> {
    match kind {
        NetworkPlayerAnimationKind::Idle => Some(paths.idle_animation_path),
        NetworkPlayerAnimationKind::Walk => paths.walk_animation_path,
    }
}

fn rotate_players(
    mut rotated: MessageReader<PlayerRotationChangedReceived>,
    mut pending: ResMut<PendingBlockyPlayerSpawns>,
    mut rendered: ResMut<RenderedNetworkPlayers>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
    gravity: Res<Gravity>,
    paths: Res<ClientPlayerBlockyModelPaths>,
) {
    for rotated in rotated.read() {
        if let Some(spawn_id) = pending.by_player_id.get(&rotated.0.player_id).copied()
            && let Some(pending_spawn) = pending.by_spawn_id.get_mut(&spawn_id)
        {
            pending_spawn.player.yaw = rotated.0.yaw;
            pending_spawn.player.pitch = rotated.0.pitch;
            pending_spawn.last_seen_at = time.elapsed_secs_f64();
        }

        let Some(visual) = rendered.entities.get_mut(&rotated.0.player_id) else {
            continue;
        };
        visual.last_seen_at = time.elapsed_secs_f64();
        if let Ok(mut avatar) = transforms.get_mut(visual.avatar) {
            avatar.rotation = avatar_rotation(*gravity, rotated.0.yaw, paths.yaw_offset_radians);
        }
    }
}

fn project_labels_to_viewport(
    camera: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    gravity: Res<Gravity>,
    rendered: Res<RenderedNetworkPlayers>,
    avatars: Query<&GlobalTransform>,
    mut labels: Query<(&mut Node, &mut Visibility)>,
) {
    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };
    for visual in rendered.entities.values() {
        let (Ok(avatar), Ok((mut node, mut visibility))) =
            (avatars.get(visual.avatar), labels.get_mut(visual.label))
        else {
            continue;
        };
        let world_position = avatar.translation() + gravity.up() * 2.0;
        match camera.world_to_viewport(camera_transform, world_position) {
            Ok(viewport) => {
                node.left = px(viewport.x);
                node.top = px(viewport.y);
                *visibility = Visibility::Inherited;
            }
            Err(_) => {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn remove_players(
    mut commands: Commands,
    mut left: MessageReader<PlayerLeftReceived>,
    mut pending: ResMut<PendingBlockyPlayerSpawns>,
    mut rendered: ResMut<RenderedNetworkPlayers>,
) {
    for left in left.read() {
        if let Some(spawn_id) = pending.by_player_id.remove(&left.0.player_id) {
            pending.by_spawn_id.remove(&spawn_id);
        }
        if let Some(visual) = rendered.entities.remove(&left.0.player_id) {
            commands.entity(visual.avatar).try_despawn();
            commands.entity(visual.label).try_despawn();
        }
    }
}

fn remove_stale_players(
    mut commands: Commands,
    time: Res<Time>,
    mut pending: ResMut<PendingBlockyPlayerSpawns>,
    mut rendered: ResMut<RenderedNetworkPlayers>,
) {
    let now = time.elapsed_secs_f64();
    let stale_pending = pending
        .by_spawn_id
        .iter()
        .filter_map(|(spawn_id, spawn)| {
            (now - spawn.last_seen_at > 5.0).then_some((*spawn_id, spawn.player.id))
        })
        .collect::<Vec<_>>();
    for (spawn_id, player_id) in stale_pending {
        pending.by_spawn_id.remove(&spawn_id);
        pending.by_player_id.remove(&player_id);
    }

    rendered.entities.retain(|_, visual| {
        if now - visual.last_seen_at <= 5.0 {
            true
        } else {
            commands.entity(visual.avatar).try_despawn();
            commands.entity(visual.label).try_despawn();
            false
        }
    });
}

fn clear_rendered_players(
    mut pending: ResMut<PendingBlockyPlayerSpawns>,
    mut rendered: ResMut<RenderedNetworkPlayers>,
) {
    pending.by_spawn_id.clear();
    pending.by_player_id.clear();
    rendered.entities.clear();
}

fn avatar_rotation(gravity: Gravity, yaw: f32, yaw_offset_radians: f32) -> Quat {
    Quat::from_axis_angle(gravity.up(), yaw + yaw_offset_radians) * gravity.alignment()
}
