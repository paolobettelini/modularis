use bevy_mod::BevyMod;
use server_biome_api::{ServerBiomeApi, ServerBiomeRegistry};
use tokio::task::JoinHandle;

pub struct ServerBiomeRegistryMod;

impl ServerBiomeRegistryMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.init_resource::<ServerBiomeRegistry>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerBiomeApi for ServerBiomeRegistryMod {}
