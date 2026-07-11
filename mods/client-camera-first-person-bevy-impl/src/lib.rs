use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_camera_api::{CameraAngles, CameraApi, PlayerCamera};
use client_game_state_api::{GameState, GameStateApi, InGameOverlayState};
use client_input_api::{InputApi, PlayerInput};
use client_settings_api::{SettingsApi, SettingsStore};
use generated_client_settings_registry::SettingKey;
use player_gravity_api::{Gravity, PlayerGravityApi};
use std::f32::consts::FRAC_PI_2;
use tokio::task::JoinHandle;

pub struct FirstPersonCameraBevyImpl;

impl FirstPersonCameraBevyImpl {
    pub fn init<I: InputApi, S: SettingsApi, V: PlayerGravityApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _input: &mut I,
        _settings: &mut S,
        _gravity: &mut V,
        _game_state: &mut G,
    ) -> Self {
        bevy.app
            .add_systems(OnEnter(GameState::InGame), spawn_camera)
            .add_systems(
                Update,
                update_look.run_if(in_state(InGameOverlayState::Playing)),
            )
            .add_systems(
                Update,
                (apply_camera_rotation, update_fov).run_if(in_state(GameState::InGame)),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl CameraApi for FirstPersonCameraBevyImpl {}

fn spawn_camera(
    mut commands: Commands,
    settings: Res<SettingsStore>,
    gravity: Res<Gravity>,
    cameras: Query<(), With<PlayerCamera>>,
) {
    if !cameras.is_empty() {
        return;
    }
    let fov = configured_fov(&settings);
    let angles = CameraAngles::default();
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection { fov, ..default() }),
        Transform {
            translation: Vec3::Y * 3.5,
            rotation: look_rotation(angles, *gravity),
            ..default()
        },
        PlayerCamera,
        angles,
        DespawnOnExit(GameState::InGame),
    ));
}

fn update_fov(
    settings: Res<SettingsStore>,
    mut camera: Query<&mut Projection, With<PlayerCamera>>,
) {
    let Ok(mut projection) = camera.single_mut() else {
        return;
    };
    if let Projection::Perspective(perspective) = projection.as_mut() {
        perspective.fov = configured_fov(&settings);
    }
}

fn configured_fov(settings: &SettingsStore) -> f32 {
    settings
        .get_f32(SettingKey::GraphicsFov)
        .unwrap_or(75.0)
        .clamp(30.0, 120.0)
        .to_radians()
}

fn update_look(
    input: Res<PlayerInput>,
    settings: Res<SettingsStore>,
    mut camera: Query<&mut CameraAngles, With<PlayerCamera>>,
) {
    let Ok(mut angles) = camera.single_mut() else {
        return;
    };
    let sensitivity = settings
        .get_f32(SettingKey::ControlsMouseSensitivity)
        .unwrap_or(0.15)
        .to_radians();
    angles.yaw -= input.look_delta.x * sensitivity;
    angles.pitch = (angles.pitch - input.look_delta.y * sensitivity)
        .clamp(-FRAC_PI_2 + 0.01, FRAC_PI_2 - 0.01);
}

fn apply_camera_rotation(
    gravity: Res<Gravity>,
    mut camera: Query<(&mut Transform, &CameraAngles), With<PlayerCamera>>,
) {
    let Ok((mut transform, angles)) = camera.single_mut() else {
        return;
    };
    transform.rotation = look_rotation(*angles, *gravity);
}

fn look_rotation(angles: CameraAngles, gravity: Gravity) -> Quat {
    let up = gravity.up();
    let base = gravity.alignment();
    let yaw = Quat::from_axis_angle(up, angles.yaw);
    let right = (yaw * base * Vec3::X).normalize_or_zero();
    let pitch = if right.length_squared() == 0.0 {
        Quat::IDENTITY
    } else {
        Quat::from_axis_angle(right, angles.pitch)
    };
    pitch * yaw * base
}
