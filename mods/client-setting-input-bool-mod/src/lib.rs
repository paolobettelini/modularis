use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_menu_api::MenuWidget;
use client_settings_input_api::{
    SettingInputContext, SettingInputRegistryHandle, SettingInputStartupSet, SettingsInputApi,
};
use tokio::task::JoinHandle;

pub struct ClientSettingInputBoolMod;

impl ClientSettingInputBoolMod {
    pub fn init<I: SettingsInputApi>(bevy: &mut BevyMod, _inputs: &mut I) -> Self {
        bevy.app.add_systems(
            Startup,
            register_bool_input.in_set(SettingInputStartupSet::RegisterInputs),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn register_bool_input(registry: Res<SettingInputRegistryHandle>) {
    registry.register("bool", |context: SettingInputContext| {
        MenuWidget::ToggleInput {
            id: context.id,
            label: context.label,
            value: context.value.parse().unwrap_or(false),
            action: context.action,
        }
    });
}
