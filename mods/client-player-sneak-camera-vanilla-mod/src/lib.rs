use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_camera_api::{CameraApi, PlayerCamera};
use client_game_state_api::{GameState, GameStateApi};
use client_player_controller_api::{PlayerControllerApi, PlayerControllerSet};
use player_gravity_api::{Gravity, PlayerGravityApi};
use player_sneak_api::{LocalPlayerSneak, PlayerSneakApi, PlayerSneakSet};
use tokio::task::JoinHandle;

#[derive(Resource, Debug, Clone, Copy)]
pub struct SneakCameraConfig {
    pub eye_offset: f32,
}

impl Default for SneakCameraConfig {
    fn default() -> Self {
        Self { eye_offset: 0.2 }
    }
}

pub struct ClientPlayerSneakCameraVanillaMod;

impl ClientPlayerSneakCameraVanillaMod {
    pub fn init<
        G: GameStateApi,
        C: CameraApi,
        P: PlayerControllerApi,
        V: PlayerGravityApi,
        S: PlayerSneakApi,
    >(
        bevy: &mut BevyMod,
        _game_state: &mut G,
        _camera: &mut C,
        _controller: &mut P,
        _gravity: &mut V,
        _sneak: &mut S,
    ) -> Self {
        bevy.app.init_resource::<SneakCameraConfig>().add_systems(
            Update,
            lower_sneaking_camera
                .in_set(PlayerControllerSet::CameraModifiers)
                .after(PlayerSneakSet::Input)
                .run_if(in_state(GameState::InGame)),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn lower_sneaking_camera(
    sneak: Res<LocalPlayerSneak>,
    config: Res<SneakCameraConfig>,
    gravity: Res<Gravity>,
    mut camera: Query<&mut Transform, With<PlayerCamera>>,
) {
    if !sneak.active {
        return;
    }
    let Ok(mut camera) = camera.single_mut() else {
        return;
    };
    camera.translation -= gravity.up() * config.eye_offset.max(0.0);
}
