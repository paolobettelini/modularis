use bevy_mod::BevyMod;
use client_grass_interaction_api::{ClientGrassInteractionApi, ClientGrassInteractionField};
use tokio::task::JoinHandle;

pub struct ClientGrassInteractionStateMod;

impl ClientGrassInteractionStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.init_resource::<ClientGrassInteractionField>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientGrassInteractionApi for ClientGrassInteractionStateMod {}
