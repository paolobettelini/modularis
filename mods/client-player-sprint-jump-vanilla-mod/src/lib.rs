use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_camera_api::{CameraApi, PlayerCamera};
use client_game_state_api::{GameStateApi, InGameOverlayState};
use client_keybinding_api::parse_key_code;
use client_player_controller_api::{
    Player, PlayerControllerApi, PlayerControllerSet, PlayerPlanarMovementIntent, PlayerVelocity,
};
use client_settings_api::{SettingsApi, SettingsStore};
use client_settings_registry_codegen::SettingsRegistryCodegenMod;
use generated_client_settings_registry::SettingKey;
use player_gravity_api::{Gravity, PlayerGravityApi, project_on_gravity_plane};
use player_jump_api::{LocalPlayerJumped, PlayerJumpApi};
use tokio::task::JoinHandle;

const SPRINT_JUMP_IMPULSE: f32 = 0.20 * 20.0;

pub struct ClientPlayerSprintJumpVanillaMod;

impl ClientPlayerSprintJumpVanillaMod {
    pub fn init<
        P: PlayerControllerApi,
        J: PlayerJumpApi,
        C: CameraApi,
        V: PlayerGravityApi,
        S: SettingsApi,
        G: GameStateApi,
    >(
        bevy: &mut BevyMod,
        _player: &mut P,
        _jump: &mut J,
        _camera: &mut C,
        _gravity: &mut V,
        _settings: &mut S,
        _codegen: &mut SettingsRegistryCodegenMod,
        _game_state: &mut G,
    ) -> Self {
        bevy.app.add_systems(
            FixedUpdate,
            apply_sprint_jump_impulse
                .in_set(PlayerControllerSet::Forces)
                .run_if(in_state(InGameOverlayState::Playing)),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_sprint_jump_impulse(
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<SettingsStore>,
    intent: Res<PlayerPlanarMovementIntent>,
    gravity: Res<Gravity>,
    camera: Query<&Transform, With<PlayerCamera>>,
    mut jumped: MessageReader<LocalPlayerJumped>,
    mut players: Query<&mut PlayerVelocity, With<Player>>,
) {
    if jumped.read().next().is_none() || intent.direction.length_squared() == 0.0 {
        return;
    }
    let sprint_key = settings
        .get_string(SettingKey::ControlsSprintKey)
        .and_then(parse_key_code)
        .unwrap_or(KeyCode::ControlLeft);
    if !keyboard.pressed(sprint_key) {
        return;
    }
    let Ok(camera) = camera.single() else {
        return;
    };
    let forward =
        project_on_gravity_plane(camera.rotation * Vec3::NEG_Z, gravity.0).normalize_or_zero();
    if forward.length_squared() == 0.0 {
        return;
    }
    for mut velocity in &mut players {
        velocity.0 += forward * SPRINT_JUMP_IMPULSE;
    }
}
