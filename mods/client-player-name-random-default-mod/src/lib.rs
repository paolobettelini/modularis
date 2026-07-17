use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_settings_api::{SettingsApi, SettingsStore};
use client_settings_input_api::{SettingInputStartupSet, SettingsInputApi};
use client_settings_registry_codegen::SettingsRegistryCodegenMod;
use generated_client_settings_registry::SettingKey;
use settings_schema_api::SettingValue;
use std::hash::{BuildHasher, Hasher, RandomState};
use tokio::task::JoinHandle;

pub struct ClientPlayerNameRandomDefaultMod;

impl ClientPlayerNameRandomDefaultMod {
    pub fn init<S: SettingsApi, I: SettingsInputApi>(
        bevy: &mut BevyMod,
        _settings: &mut S,
        _inputs: &mut I,
        _codegen: &mut SettingsRegistryCodegenMod,
    ) -> Self {
        bevy.app.add_systems(
            Startup,
            choose_random_default_name.before(SettingInputStartupSet::BuildMenus),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn choose_random_default_name(mut settings: ResMut<SettingsStore>) {
    if settings.get_string(SettingKey::NetworkPlayerName) != Some("Player") {
        return;
    }
    let suffix = RandomState::new().build_hasher().finish() % 101;
    let _ = settings.set(
        SettingKey::NetworkPlayerName,
        SettingValue::String(format!("Player{suffix}")),
    );
}
