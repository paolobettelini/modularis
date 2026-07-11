use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_config_api::ClientConfigApi;
use client_log_filter_api::ClientLogFilterApi;
use tokio::task::JoinHandle;

pub struct ClientBevyDefaultPluginsMod;

impl ClientBevyDefaultPluginsMod {
    pub fn init<C: ClientConfigApi, L: ClientLogFilterApi>(
        bevy: &mut BevyMod,
        _config: &mut C,
        _log_filter: &mut L,
    ) -> Self {
        bevy.app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(bevy::log::LogPlugin {
                    filter: L::filter().to_string(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: C::window_title().to_string(),
                        resolution: (1280, 720).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
