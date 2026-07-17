use bevy_mod::BevyMod;
use player_hitbox_api::{PlayerHitbox, PlayerHitboxApi};
use tokio::task::JoinHandle;

pub struct ClientPlayerHitboxStateMod;

impl ClientPlayerHitboxStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.init_resource::<PlayerHitbox>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl PlayerHitboxApi for ClientPlayerHitboxStateMod {}
