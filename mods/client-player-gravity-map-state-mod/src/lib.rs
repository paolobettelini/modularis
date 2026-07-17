use bevy_mod::BevyMod;
use client_player_gravity_map_api::{
    ClientPlayerGravities, ClientPlayerGravityChanged, ClientPlayerGravityMapApi,
};
use tokio::task::JoinHandle;

pub struct ClientPlayerGravityMapStateMod;

impl ClientPlayerGravityMapStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .init_resource::<ClientPlayerGravities>()
            .add_message::<ClientPlayerGravityChanged>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientPlayerGravityMapApi for ClientPlayerGravityMapStateMod {}
