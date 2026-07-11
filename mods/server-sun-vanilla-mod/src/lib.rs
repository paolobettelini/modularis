use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_sun_api::{ServerSunApi, SetServerSun};
use sun_api::SunSettings;
use tokio::task::JoinHandle;

pub struct ServerSunVanillaMod;

impl ServerSunVanillaMod {
    pub fn init<S: ServerSunApi>(bevy: &mut BevyMod, _sun: &mut S) -> Self {
        bevy.app.add_systems(Startup, configure_vanilla_sun);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn configure_vanilla_sun(mut settings: MessageWriter<SetServerSun>) {
    settings.write(SetServerSun {
        settings: SunSettings {
            position: [0.45, 0.82, 0.35],
            illuminance: 12_000.0,
            color: [1.0, 0.94, 0.82],
        },
    });
}
