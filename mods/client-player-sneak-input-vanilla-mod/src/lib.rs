use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameStateApi, InGameOverlayState};
use client_keybinding_api::parse_key_code;
use client_settings_api::{SettingsApi, SettingsStore};
use client_settings_registry_codegen::SettingsRegistryCodegenMod;
use generated_client_settings_registry::SettingKey;
use player_sneak_api::{LocalPlayerSneak, LocalPlayerSneakChanged, PlayerSneakApi, PlayerSneakSet};
use tokio::task::JoinHandle;

pub struct ClientPlayerSneakInputVanillaMod;

impl ClientPlayerSneakInputVanillaMod {
    pub fn init<G: GameStateApi, P: PlayerSneakApi, S: SettingsApi>(
        bevy: &mut BevyMod,
        _game_state: &mut G,
        _sneak: &mut P,
        _settings: &mut S,
        _codegen: &mut SettingsRegistryCodegenMod,
    ) -> Self {
        bevy.app
            .add_systems(
                Update,
                update_sneak_input
                    .in_set(PlayerSneakSet::Input)
                    .run_if(in_state(InGameOverlayState::Playing)),
            )
            .add_systems(OnExit(InGameOverlayState::Playing), clear_sneak_input);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn update_sneak_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<SettingsStore>,
    mut sneak: ResMut<LocalPlayerSneak>,
    mut changed: MessageWriter<LocalPlayerSneakChanged>,
) {
    let key = settings
        .get_string(SettingKey::ControlsSneakKey)
        .and_then(parse_key_code)
        .unwrap_or(KeyCode::ShiftLeft);
    set_sneaking(keyboard.pressed(key), &mut sneak, &mut changed);
}

fn clear_sneak_input(
    mut sneak: ResMut<LocalPlayerSneak>,
    mut changed: MessageWriter<LocalPlayerSneakChanged>,
) {
    set_sneaking(false, &mut sneak, &mut changed);
}

fn set_sneaking(
    active: bool,
    sneak: &mut LocalPlayerSneak,
    changed: &mut MessageWriter<LocalPlayerSneakChanged>,
) {
    if sneak.active == active {
        return;
    }
    sneak.active = active;
    changed.write(LocalPlayerSneakChanged { active });
}
