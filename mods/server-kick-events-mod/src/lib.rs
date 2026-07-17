use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_kick_api::{ServerKickApi, ServerKickRequested, ServerKickSet};
use tokio::task::JoinHandle;

pub struct ServerKickEventsMod;

impl ServerKickEventsMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .add_message::<ServerKickRequested>()
            .configure_sets(Update, ServerKickSet::Apply);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerKickApi for ServerKickEventsMod {}
