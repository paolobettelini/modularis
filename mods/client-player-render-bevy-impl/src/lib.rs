use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_camera_api::{CameraApi, PlayerCamera};
use client_game_state_api::{GameState, GameStateApi};
use client_player_gravity_map_api::{
    ClientPlayerGravities, ClientPlayerGravityMapApi, ClientPlayerGravityMapSet,
};
use client_player_render_api::{
    ClientPlayerRenderApi, NetworkPlayerVisual, RenderedNetworkPlayers,
};
use client_player_scale_map_api::{
    ClientPlayerScaleMapApi, ClientPlayerScaleMapSet, ClientPlayerScales,
};
use generated_network_messages::{
    JoinAcceptedReceived, NetworkMessageSet, PlayerJoinedReceived, PlayerLeftReceived,
    PlayerMovedReceived, PlayerRotationChangedReceived,
};
use network_protocol_mod::NetworkProtocolMod;
use player_gravity_api::Gravity;
use player_network_message_types::NetworkPlayer;
use tokio::task::JoinHandle;

#[derive(Resource)]
struct PlayerVisualAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

pub struct ClientPlayerRenderBevyImpl;

impl ClientPlayerRenderBevyImpl {
    pub fn init<
        C: CameraApi,
        V: ClientPlayerGravityMapApi,
        S: ClientPlayerScaleMapApi,
        G: GameStateApi,
    >(
        bevy: &mut BevyMod,
        _camera: &mut C,
        _gravity_map: &mut V,
        _scale_map: &mut S,
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
                    sync_player_attributes,
                    project_labels_to_viewport,
                    remove_players,
                    remove_stale_players,
                )
                    .chain()
                    .after(NetworkMessageSet::DispatchPackets)
                    .after(ClientPlayerGravityMapSet)
                    .after(ClientPlayerScaleMapSet)
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
    gravities: Res<ClientPlayerGravities>,
    scales: Res<ClientPlayerScales>,
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
                gravities.gravity(player.id),
                scales.scale(player.id),
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
    gravities: Res<ClientPlayerGravities>,
    scales: Res<ClientPlayerScales>,
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
            gravities.gravity(joined.0.player.id),
            scales.scale(joined.0.player.id),
        );
    }
}

fn move_players(
    mut moved: MessageReader<PlayerMovedReceived>,
    mut rendered: ResMut<RenderedNetworkPlayers>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
    gravities: Res<ClientPlayerGravities>,
) {
    for moved in moved.read() {
        let Some(visual) = rendered.entities.get_mut(&moved.0.player_id) else {
            continue;
        };
        visual.last_seen_at = time.elapsed_secs_f64();
        visual.yaw = moved.0.yaw;
        let position = Vec3::from_array(moved.0.position);
        if let Ok(mut avatar) = transforms.get_mut(visual.avatar) {
            avatar.translation = position;
            avatar.rotation = avatar_rotation(gravities.gravity(moved.0.player_id), moved.0.yaw);
        }
    }
}

fn rotate_players(
    mut rotated: MessageReader<PlayerRotationChangedReceived>,
    mut rendered: ResMut<RenderedNetworkPlayers>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
    gravities: Res<ClientPlayerGravities>,
) {
    for rotated in rotated.read() {
        let Some(visual) = rendered.entities.get_mut(&rotated.0.player_id) else {
            continue;
        };
        visual.last_seen_at = time.elapsed_secs_f64();
        visual.yaw = rotated.0.yaw;
        if let Ok(mut avatar) = transforms.get_mut(visual.avatar) {
            avatar.rotation =
                avatar_rotation(gravities.gravity(rotated.0.player_id), rotated.0.yaw);
        }
    }
}

fn sync_player_attributes(
    rendered: Res<RenderedNetworkPlayers>,
    gravities: Res<ClientPlayerGravities>,
    scales: Res<ClientPlayerScales>,
    mut transforms: Query<&mut Transform>,
) {
    if !gravities.is_changed() && !scales.is_changed() {
        return;
    }
    for (player_id, visual) in &rendered.entities {
        let Ok(mut avatar) = transforms.get_mut(visual.avatar) else {
            continue;
        };
        avatar.rotation = avatar_rotation(gravities.gravity(*player_id), visual.yaw);
        avatar.scale = Vec3::splat(scales.scale(*player_id));
    }
}

fn project_labels_to_viewport(
    camera: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    gravities: Res<ClientPlayerGravities>,
    scales: Res<ClientPlayerScales>,
    rendered: Res<RenderedNetworkPlayers>,
    avatars: Query<&GlobalTransform>,
    mut labels: Query<(&mut Node, &mut Visibility)>,
) {
    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };
    for (player_id, visual) in &rendered.entities {
        let (Ok(avatar), Ok((mut node, mut visibility))) =
            (avatars.get(visual.avatar), labels.get_mut(visual.label))
        else {
            continue;
        };
        let world_position = avatar.translation()
            + gravities.gravity(*player_id).up() * 2.0 * scales.scale(*player_id);
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
    player_scale: f32,
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
                scale: Vec3::splat(player_scale),
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
            yaw: player.yaw,
        },
    );
}

fn avatar_rotation(gravity: Gravity, yaw: f32) -> Quat {
    Quat::from_axis_angle(gravity.up(), yaw) * gravity.alignment()
}
