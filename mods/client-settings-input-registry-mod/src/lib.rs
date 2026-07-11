use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_settings_input_api::{
    SettingInputRegistryHandle, SettingInputStartupSet, SettingsInputApi,
};
use tokio::task::JoinHandle;

pub struct ClientSettingsInputRegistryMod;

impl ClientSettingsInputRegistryMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .init_resource::<SettingInputRegistryHandle>()
            .configure_sets(
                Startup,
                (
                    SettingInputStartupSet::RegisterInputs,
                    SettingInputStartupSet::BuildMenus,
                )
                    .chain(),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl SettingsInputApi for ClientSettingsInputRegistryMod {}
