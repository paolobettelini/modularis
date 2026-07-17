use bevy_mod::BevyMod;
use client_player_scale_map_api::{
    ClientPlayerScaleChanged, ClientPlayerScaleMapApi, ClientPlayerScales,
};
use tokio::task::JoinHandle;

pub struct ClientPlayerScaleMapStateMod;

impl ClientPlayerScaleMapStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .init_resource::<ClientPlayerScales>()
            .add_message::<ClientPlayerScaleChanged>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientPlayerScaleMapApi for ClientPlayerScaleMapStateMod {}
