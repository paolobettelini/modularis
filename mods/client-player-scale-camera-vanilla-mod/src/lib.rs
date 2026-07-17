use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_camera_api::{CameraApi, PlayerCamera};
use client_game_state_api::{GameState, GameStateApi};
use client_player_controller_api::{PLAYER_EYE_HEIGHT, PlayerControllerApi, PlayerControllerSet};
use player_gravity_api::{Gravity, PlayerGravityApi};
use player_scale_api::{PlayerScale, PlayerScaleApi};
use tokio::task::JoinHandle;

pub struct ClientPlayerScaleCameraVanillaMod;

impl ClientPlayerScaleCameraVanillaMod {
    pub fn init<
        G: GameStateApi,
        C: CameraApi,
        P: PlayerControllerApi,
        V: PlayerGravityApi,
        S: PlayerScaleApi,
    >(
        bevy: &mut BevyMod,
        _game_state: &mut G,
        _camera: &mut C,
        _controller: &mut P,
        _gravity: &mut V,
        _scale: &mut S,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            apply_scaled_eye_height
                .in_set(PlayerControllerSet::CameraModifiers)
                .run_if(in_state(GameState::InGame)),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_scaled_eye_height(
    gravity: Res<Gravity>,
    scale: Res<PlayerScale>,
    mut camera: Query<&mut Transform, With<PlayerCamera>>,
) {
    let Ok(mut camera) = camera.single_mut() else {
        return;
    };
    camera.translation += gravity.up() * PLAYER_EYE_HEIGHT * (scale.0 - 1.0);
}
