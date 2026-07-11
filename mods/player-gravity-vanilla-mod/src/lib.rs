use bevy_mod::BevyMod;
use player_gravity_api::{Gravity, PlayerGravityApi};
use tokio::task::JoinHandle;

pub struct VanillaPlayerGravityMod;

impl VanillaPlayerGravityMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.init_resource::<Gravity>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl PlayerGravityApi for VanillaPlayerGravityMod {}
