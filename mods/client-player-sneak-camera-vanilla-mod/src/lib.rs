use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_camera_api::{CameraApi, PlayerCamera};
use client_game_state_api::{GameState, GameStateApi};
use client_player_controller_api::{PlayerControllerApi, PlayerControllerSet};
use player_gravity_api::{Gravity, PlayerGravityApi};
use player_scale_api::{PlayerScale, PlayerScaleApi};
use player_sneak_api::{LocalPlayerSneak, PlayerSneakApi, PlayerSneakSet};
use tokio::task::JoinHandle;

#[derive(Resource, Debug, Clone, Copy)]
pub struct SneakCameraConfig {
    pub eye_offset: f32,
    pub transition_seconds: f32,
}

impl Default for SneakCameraConfig {
    fn default() -> Self {
        Self {
            eye_offset: 0.2,
            transition_seconds: 0.12,
        }
    }
}

#[derive(Resource, Debug, Default)]
struct SneakCameraOffset(f32);

pub struct ClientPlayerSneakCameraVanillaMod;

impl ClientPlayerSneakCameraVanillaMod {
    pub fn init<
        G: GameStateApi,
        C: CameraApi,
        P: PlayerControllerApi,
        V: PlayerGravityApi,
        Z: PlayerScaleApi,
        S: PlayerSneakApi,
    >(
        bevy: &mut BevyMod,
        _game_state: &mut G,
        _camera: &mut C,
        _controller: &mut P,
        _gravity: &mut V,
        _scale: &mut Z,
        _sneak: &mut S,
    ) -> Self {
        bevy.app
            .init_resource::<SneakCameraConfig>()
            .init_resource::<SneakCameraOffset>()
            .add_systems(
                Update,
                lower_sneaking_camera
                    .in_set(PlayerControllerSet::CameraModifiers)
                    .after(PlayerSneakSet::Input)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), reset_sneak_camera_offset);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn lower_sneaking_camera(
    sneak: Res<LocalPlayerSneak>,
    config: Res<SneakCameraConfig>,
    scale: Res<PlayerScale>,
    gravity: Res<Gravity>,
    time: Res<Time>,
    mut offset: ResMut<SneakCameraOffset>,
    mut camera: Query<&mut Transform, With<PlayerCamera>>,
) {
    let scaled_eye_offset = config.eye_offset.max(0.0) * scale.0.max(0.0);
    let target = if sneak.active { scaled_eye_offset } else { 0.0 };
    let transition = config.transition_seconds.max(1.0e-3);
    let transition_distance = scaled_eye_offset.max(offset.0).max(target).max(1.0e-3);
    let maximum_step = transition_distance / transition * time.delta_secs();
    offset.0 = move_towards(offset.0, target, maximum_step);
    let Ok(mut camera) = camera.single_mut() else {
        return;
    };
    camera.translation -= gravity.up() * offset.0;
}

fn move_towards(current: f32, target: f32, maximum_step: f32) -> f32 {
    if current < target {
        (current + maximum_step).min(target)
    } else {
        (current - maximum_step).max(target)
    }
}

fn reset_sneak_camera_offset(mut offset: ResMut<SneakCameraOffset>) {
    offset.0 = 0.0;
}
