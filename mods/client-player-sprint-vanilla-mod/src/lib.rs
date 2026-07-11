use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameStateApi, InGameOverlayState};
use client_keybinding_api::parse_key_code;
use client_player_controller_api::{
    PlayerControllerApi, PlayerControllerSet, PlayerPlanarMovementIntent,
};
use client_settings_api::{SettingsApi, SettingsStore};
use client_settings_registry_codegen::SettingsRegistryCodegenMod;
use generated_client_settings_registry::SettingKey;
use tokio::task::JoinHandle;

pub struct ClientPlayerSprintVanillaMod;

#[derive(Resource, Debug, Clone, Copy)]
pub struct SprintConfig {
    pub speed_multiplier: f32,
}

impl Default for SprintConfig {
    fn default() -> Self {
        Self {
            speed_multiplier: 1.55,
        }
    }
}

impl ClientPlayerSprintVanillaMod {
    pub fn init<G: GameStateApi, P: PlayerControllerApi, S: SettingsApi>(
        bevy: &mut BevyMod,
        _game_state: &mut G,
        _controller: &mut P,
        _settings: &mut S,
        _codegen: &mut SettingsRegistryCodegenMod,
    ) -> Self {
        bevy.app.init_resource::<SprintConfig>().add_systems(
            Update,
            apply_sprint_modifier
                .in_set(PlayerControllerSet::MovementModifiers)
                .run_if(in_state(InGameOverlayState::Playing)),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_sprint_modifier(
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<SettingsStore>,
    config: Res<SprintConfig>,
    mut movement: ResMut<PlayerPlanarMovementIntent>,
) {
    let sprint_key = settings
        .get_string(SettingKey::ControlsSprintKey)
        .and_then(parse_key_code)
        .unwrap_or(KeyCode::ControlLeft);
    if keyboard.pressed(sprint_key) && movement.direction.length_squared() > 0.0 {
        movement.speed_multiplier *= config.speed_multiplier;
    }
}
