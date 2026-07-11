use bevy_mod::BevyMod;
use server_chunk_provider_api::ServerChunkProviderRegistry;
use tokio::task::JoinHandle;

pub struct ServerChunkProviderRegistryMod;

impl ServerChunkProviderRegistryMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.init_resource::<ServerChunkProviderRegistry>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
