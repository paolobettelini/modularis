use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_menu_api::MenuWidget;
use client_settings_input_api::{
    SettingInputContext, SettingInputRegistryHandle, SettingInputStartupSet, SettingsInputApi,
};
use tokio::task::JoinHandle;

pub struct ClientSettingInputKeybindingMod;

impl ClientSettingInputKeybindingMod {
    pub fn init<I: SettingsInputApi>(bevy: &mut BevyMod, _inputs: &mut I) -> Self {
        bevy.app.add_systems(
            Startup,
            register_keybinding_input.in_set(SettingInputStartupSet::RegisterInputs),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn register_keybinding_input(registry: Res<SettingInputRegistryHandle>) {
    registry.register("keybinding", |context: SettingInputContext| {
        MenuWidget::KeybindingInput {
            id: context.id,
            label: context.label,
            value: context.value,
            action: context.action,
        }
    });
}
