use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_sky_api::{ClientSkyApi, ClientSkyColor};
use tokio::task::JoinHandle;

pub struct ClientSkyBevyMod;

impl ClientSkyBevyMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .init_resource::<ClientSkyColor>()
            .init_resource::<ClearColor>()
            .add_systems(Update, apply_sky_color);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientSkyApi for ClientSkyBevyMod {}

fn apply_sky_color(sky: Res<ClientSkyColor>, mut clear: ResMut<ClearColor>) {
    if !sky.is_changed() {
        return;
    }
    clear.0 = Color::srgba(sky.0[0], sky.0[1], sky.0[2], sky.0[3]);
}
