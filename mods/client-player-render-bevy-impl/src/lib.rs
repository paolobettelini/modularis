use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_camera_api::{CameraApi, PlayerCamera};
use client_game_state_api::{GameState, GameStateApi};
use client_player_render_api::{
    ClientPlayerRenderApi, NetworkPlayerVisual, RenderedNetworkPlayers,
};
use generated_network_messages::{
    JoinAcceptedReceived, NetworkMessageSet, PlayerJoinedReceived, PlayerLeftReceived,
    PlayerMovedReceived, PlayerRotationChangedReceived,
};
use network_protocol_mod::NetworkProtocolMod;
use player_gravity_api::{Gravity, PlayerGravityApi};
use player_network_message_types::NetworkPlayer;
use tokio::task::JoinHandle;

#[derive(Resource)]
struct PlayerVisualAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

pub struct ClientPlayerRenderBevyImpl;

impl ClientPlayerRenderBevyImpl {
    pub fn init<C: CameraApi, V: PlayerGravityApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _camera: &mut C,
        _gravity: &mut V,
        _game_state: &mut G,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app
            .init_resource::<RenderedNetworkPlayers>()
            .add_systems(
                Update,
                (
                    initialize_visual_assets,
                    render_initial_players,
                    render_joined_players,
                    move_players,
                    rotate_players,
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

impl ClientPlayerRenderApi for ClientPlayerRenderBevyImpl {}

fn initialize_visual_assets(
    mut commands: Commands,
    assets: Option<Res<PlayerVisualAssets>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if assets.is_some() {
        return;
    }
    commands.insert_resource(PlayerVisualAssets {
        mesh: meshes.add(Sphere::new(0.5).mesh().ico(3).unwrap()),
        material: materials.add(Color::srgb(0.95, 0.55, 0.15)),
    });
}

fn render_initial_players(
    mut commands: Commands,
    assets: Option<Res<PlayerVisualAssets>>,
    mut accepted: MessageReader<JoinAcceptedReceived>,
    mut rendered: ResMut<RenderedNetworkPlayers>,
    time: Res<Time>,
    gravity: Res<Gravity>,
) {
    let Some(assets) = assets else {
        return;
    };
    for accepted in accepted.read() {
        for player in &accepted.0.players {
            if player.id == accepted.0.player_id {
                continue;
            }
            spawn_player(
                &mut commands,
                &assets,
                &mut rendered,
                player,
                time.elapsed_secs_f64(),
                *gravity,
            );
        }
    }
}

fn render_joined_players(
    mut commands: Commands,
    assets: Option<Res<PlayerVisualAssets>>,
    mut joined: MessageReader<PlayerJoinedReceived>,
    mut rendered: ResMut<RenderedNetworkPlayers>,
    time: Res<Time>,
    gravity: Res<Gravity>,
) {
    let Some(assets) = assets else {
        return;
    };
    for joined in joined.read() {
        spawn_player(
            &mut commands,
            &assets,
            &mut rendered,
            &joined.0.player,
            time.elapsed_secs_f64(),
            *gravity,
        );
    }
}

fn move_players(
    mut moved: MessageReader<PlayerMovedReceived>,
    mut rendered: ResMut<RenderedNetworkPlayers>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
    gravity: Res<Gravity>,
) {
    for moved in moved.read() {
        let Some(visual) = rendered.entities.get_mut(&moved.0.player_id) else {
            continue;
        };
        visual.last_seen_at = time.elapsed_secs_f64();
        let position = Vec3::from_array(moved.0.position);
        if let Ok(mut avatar) = transforms.get_mut(visual.avatar) {
            avatar.translation = position;
            avatar.rotation = avatar_rotation(*gravity, moved.0.yaw);
        }
    }
}

fn rotate_players(
    mut rotated: MessageReader<PlayerRotationChangedReceived>,
    mut rendered: ResMut<RenderedNetworkPlayers>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
    gravity: Res<Gravity>,
) {
    for rotated in rotated.read() {
        let Some(visual) = rendered.entities.get_mut(&rotated.0.player_id) else {
            continue;
        };
        visual.last_seen_at = time.elapsed_secs_f64();
        if let Ok(mut avatar) = transforms.get_mut(visual.avatar) {
            avatar.rotation = avatar_rotation(*gravity, rotated.0.yaw);
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
    mut rendered: ResMut<RenderedNetworkPlayers>,
) {
    for left in left.read() {
        if let Some(visual) = rendered.entities.remove(&left.0.player_id) {
            commands.entity(visual.avatar).try_despawn();
            commands.entity(visual.label).try_despawn();
        }
    }
}

fn remove_stale_players(
    mut commands: Commands,
    time: Res<Time>,
    mut rendered: ResMut<RenderedNetworkPlayers>,
) {
    let now = time.elapsed_secs_f64();
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

fn clear_rendered_players(mut rendered: ResMut<RenderedNetworkPlayers>) {
    rendered.entities.clear();
}

fn spawn_player(
    commands: &mut Commands,
    assets: &PlayerVisualAssets,
    rendered: &mut RenderedNetworkPlayers,
    player: &NetworkPlayer,
    now: f64,
    gravity: Gravity,
) {
    if rendered.entities.contains_key(&player.id) {
        return;
    }
    let position = Vec3::from_array(player.position);
    let avatar = commands
        .spawn((
            Transform {
                translation: position,
                rotation: avatar_rotation(gravity, player.yaw),
                ..default()
            },
            Visibility::Inherited,
            DespawnOnExit(GameState::InGame),
        ))
        .with_children(|body| {
            body.spawn((
                Mesh3d(assets.mesh.clone()),
                MeshMaterial3d(assets.material.clone()),
                Transform::from_xyz(0.0, 0.5, 0.0),
            ));
            body.spawn((
                Mesh3d(assets.mesh.clone()),
                MeshMaterial3d(assets.material.clone()),
                Transform::from_xyz(0.0, 1.3, 0.0),
            ));
        })
        .id();
    let label = commands
        .spawn((
            Text::new(player.name.clone()),
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
        player.id,
        NetworkPlayerVisual {
            avatar,
            label,
            last_seen_at: now,
        },
    );
}

fn avatar_rotation(gravity: Gravity, yaw: f32) -> Quat {
    Quat::from_axis_angle(gravity.up(), yaw) * gravity.alignment()
}
