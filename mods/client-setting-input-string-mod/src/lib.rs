use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_menu_api::MenuWidget;
use client_settings_input_api::{
    SettingInputContext, SettingInputRegistryHandle, SettingInputStartupSet, SettingsInputApi,
};
use tokio::task::JoinHandle;

pub struct ClientSettingInputStringMod;

impl ClientSettingInputStringMod {
    pub fn init<I: SettingsInputApi>(bevy: &mut BevyMod, _inputs: &mut I) -> Self {
        bevy.app.add_systems(
            Startup,
            register_string_input.in_set(SettingInputStartupSet::RegisterInputs),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn register_string_input(registry: Res<SettingInputRegistryHandle>) {
    registry.register("string", |context: SettingInputContext| {
        MenuWidget::Textbox {
            id: context.id,
            label: context.label,
            value: context.value,
            action: context.action,
        }
    });
}
