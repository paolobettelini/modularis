use bevy_mod::BevyMod;
use player_sneak_api::{LocalPlayerSneak, LocalPlayerSneakChanged, PlayerSneakApi};
use tokio::task::JoinHandle;

pub struct ClientPlayerSneakStateMod;

impl ClientPlayerSneakStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .init_resource::<LocalPlayerSneak>()
            .add_message::<LocalPlayerSneakChanged>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl PlayerSneakApi for ClientPlayerSneakStateMod {}
