use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi};
use client_sun_api::{ClientSunApi, ClientSunLight};
use tokio::task::JoinHandle;

pub struct ClientSunShadowsVanillaMod;

impl ClientSunShadowsVanillaMod {
    pub fn init<S: ClientSunApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _sun: &mut S,
        _game_state: &mut G,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            enable_sun_shadows.run_if(in_state(GameState::InGame)),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn enable_sun_shadows(mut lights: Query<&mut DirectionalLight, With<ClientSunLight>>) {
    for mut light in &mut lights {
        if !light.shadows_enabled {
            light.shadows_enabled = true;
        }
    }
}
