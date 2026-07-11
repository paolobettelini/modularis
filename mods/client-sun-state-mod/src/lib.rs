use bevy_mod::BevyMod;
use client_sun_api::{ClientSunApi, ClientSunSettings, ClientSunSettingsChanged};
use tokio::task::JoinHandle;

pub struct ClientSunStateMod;

impl ClientSunStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .init_resource::<ClientSunSettings>()
            .add_message::<ClientSunSettingsChanged>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientSunApi for ClientSunStateMod {}
