use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_menu_api::{MenuNumberKind, MenuWidget};
use client_settings_input_api::{
    SettingInputContext, SettingInputRegistryHandle, SettingInputStartupSet, SettingsInputApi,
};
use tokio::task::JoinHandle;

pub struct ClientSettingInputF32Mod;

impl ClientSettingInputF32Mod {
    pub fn init<I: SettingsInputApi>(bevy: &mut BevyMod, _inputs: &mut I) -> Self {
        bevy.app.add_systems(
            Startup,
            register_f32_input.in_set(SettingInputStartupSet::RegisterInputs),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn register_f32_input(registry: Res<SettingInputRegistryHandle>) {
    registry.register("f32", |context: SettingInputContext| {
        MenuWidget::NumberInput {
            id: context.id,
            label: context.label,
            value: context.value,
            action: context.action,
            kind: MenuNumberKind::F32,
            step: 0.1,
            min: context.min,
            max: context.max,
        }
    });
}
