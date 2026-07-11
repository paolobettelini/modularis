use bevy::{
    light::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use bevy_mod::BevyMod;
use client_camera_api::{CameraApi, PlayerCamera};
use client_game_state_api::{GameState, GameStateApi};
use client_sun_api::{ClientSunApi, ClientSunSettings};
use tokio::task::JoinHandle;

const SUN_DISC_DISTANCE: f32 = 96.0;
const SUN_DISC_RADIUS: f32 = 3.0;

#[derive(Component)]
struct ClientSunDisc;

pub struct ClientSunDiscBevyMod;

impl ClientSunDiscBevyMod {
    pub fn init<S: ClientSunApi, C: CameraApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _sun: &mut S,
        _camera: &mut C,
        _game_state: &mut G,
    ) -> Self {
        bevy.app
            .add_systems(OnEnter(GameState::InGame), spawn_sun_disc)
            .add_systems(
                Update,
                (position_sun_disc, update_sun_disc_color).run_if(in_state(GameState::InGame)),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn spawn_sun_disc(
    mut commands: Commands,
    settings: Res<ClientSunSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(SUN_DISC_RADIUS).mesh().ico(2).unwrap())),
        MeshMaterial3d(materials.add(sun_material(settings.0.color))),
        Transform::default(),
        ClientSunDisc,
        NotShadowCaster,
        NotShadowReceiver,
        DespawnOnExit(GameState::InGame),
    ));
}

fn position_sun_disc(
    settings: Res<ClientSunSettings>,
    camera: Query<&GlobalTransform, With<PlayerCamera>>,
    mut discs: Query<&mut Transform, With<ClientSunDisc>>,
) {
    let Ok(camera) = camera.single() else {
        return;
    };
    let direction = sun_direction(settings.0.position);
    for mut transform in &mut discs {
        transform.translation = camera.translation() + direction * SUN_DISC_DISTANCE;
    }
}

fn update_sun_disc_color(
    settings: Res<ClientSunSettings>,
    discs: Query<&MeshMaterial3d<StandardMaterial>, With<ClientSunDisc>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !settings.is_changed() {
        return;
    }
    for material in &discs {
        if let Some(material) = materials.get_mut(&material.0) {
            *material = sun_material(settings.0.color);
        }
    }
}

fn sun_direction(position: [f32; 3]) -> Vec3 {
    Vec3::from_array(position).normalize_or(Vec3::Y)
}

fn sun_material(color: [f32; 3]) -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb(color[0], color[1], color[2]),
        unlit: true,
        ..default()
    }
}
