use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi, InGameOverlayCommand, InGameOverlayState};
use client_keybinding_api::parse_key_code;
use client_settings_api::{SettingsApi, SettingsStore};
use client_settings_registry_codegen::SettingsRegistryCodegenMod;
use generated_client_settings_registry::SettingKey;
use tokio::task::JoinHandle;

pub struct ClientInventoryToggleInputMod;

impl ClientInventoryToggleInputMod {
    pub fn init<G: GameStateApi, S: SettingsApi>(
        bevy: &mut BevyMod,
        _game_state: &mut G,
        _settings: &mut S,
        _codegen: &mut SettingsRegistryCodegenMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            inventory_key_toggle.run_if(in_state(GameState::InGame)),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn inventory_key_toggle(
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<SettingsStore>,
    overlay: Res<State<InGameOverlayState>>,
    mut commands: MessageWriter<InGameOverlayCommand>,
) {
    let inventory_key = settings
        .get_string(SettingKey::ControlsInventoryKey)
        .and_then(parse_key_code)
        .unwrap_or(KeyCode::KeyE);
    if !keyboard.just_pressed(inventory_key) {
        return;
    }
    match overlay.get() {
        InGameOverlayState::Playing => {
            commands.write(InGameOverlayCommand::OpenInventory);
        }
        InGameOverlayState::Inventory => {
            commands.write(InGameOverlayCommand::Resume);
        }
        InGameOverlayState::PauseMenu | InGameOverlayState::Settings => {}
    }
}
