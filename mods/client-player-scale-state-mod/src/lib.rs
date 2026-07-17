use bevy_mod::BevyMod;
use player_scale_api::{PlayerScale, PlayerScaleApi};
use tokio::task::JoinHandle;

pub struct ClientPlayerScaleStateMod;

impl ClientPlayerScaleStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.init_resource::<PlayerScale>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl PlayerScaleApi for ClientPlayerScaleStateMod {}
