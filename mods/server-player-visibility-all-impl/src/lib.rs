use bevy_mod::BevyMod;
use server_player_visibility_api::{ServerPlayerVisibility, ServerPlayerVisibilityApi};
use tokio::task::JoinHandle;

pub struct ServerPlayerVisibilityAllImpl;

impl ServerPlayerVisibilityAllImpl {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.insert_resource(ServerPlayerVisibility::new(|_, _| true));
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl ServerPlayerVisibilityApi for ServerPlayerVisibilityAllImpl {}

