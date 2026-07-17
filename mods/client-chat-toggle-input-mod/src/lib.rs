use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_chat_api::ClientChatApi;
use client_game_state_api::{GameState, GameStateApi, InGameOverlayCommand, InGameOverlayState};
use client_keybinding_api::parse_key_code;
use client_settings_api::{SettingsApi, SettingsStore};
use client_settings_registry_codegen::SettingsRegistryCodegenMod;
use generated_client_settings_registry::SettingKey;
use tokio::task::JoinHandle;

pub struct ClientChatToggleInputMod;

impl ClientChatToggleInputMod {
    pub fn init<G: GameStateApi, S: SettingsApi, C: ClientChatApi>(
        bevy: &mut BevyMod,
        _game_state: &mut G,
        _settings: &mut S,
        _codegen: &mut SettingsRegistryCodegenMod,
        _chat: &mut C,
    ) -> Self {
        bevy.app
            .add_systems(Update, open_chat.run_if(in_state(GameState::InGame)));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn open_chat(
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<SettingsStore>,
    overlay: Res<State<InGameOverlayState>>,
    mut commands: MessageWriter<InGameOverlayCommand>,
) {
    if *overlay.get() != InGameOverlayState::Playing {
        return;
    }
    let key = settings
        .get_string(SettingKey::ControlsChatKey)
        .and_then(parse_key_code)
        .unwrap_or(KeyCode::KeyT);
    if keyboard.just_pressed(key) {
        commands.write(InGameOverlayCommand::OpenChat);
    }
}
