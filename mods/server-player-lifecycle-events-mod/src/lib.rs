use bevy_mod::BevyMod;
use server_player_lifecycle_events_api::{ServerPlayerJoined, ServerPlayerLeft};
use tokio::task::JoinHandle;

pub struct ServerPlayerLifecycleEventsMod;

impl ServerPlayerLifecycleEventsMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .add_message::<ServerPlayerJoined>()
            .add_message::<ServerPlayerLeft>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
