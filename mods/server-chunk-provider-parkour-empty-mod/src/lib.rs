use bevy_mod::BevyMod;
use chunk_api::Chunk;
use generated_block_registry::BlockId;
use server_chunk_provider_api::{
    ChunkGenerationRequest, ChunkProviderId, ServerChunkProvider, ServerChunkProviderRegistry,
};
use server_chunk_provider_registry_mod::ServerChunkProviderRegistryMod;
use server_primary_chunk_provider_api::ServerPrimaryChunkProviderApi;
use tokio::task::JoinHandle;

pub struct ServerChunkProviderParkourEmptyMod;

impl ServerChunkProviderParkourEmptyMod {
    pub fn init(bevy: &mut BevyMod, _providers: &mut ServerChunkProviderRegistryMod) -> Self {
        bevy.app
            .world()
            .resource::<ServerChunkProviderRegistry>()
            .register(ChunkProviderId::primary(), EmptyParkourProvider)
            .expect("the primary server chunk provider must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerPrimaryChunkProviderApi for ServerChunkProviderParkourEmptyMod {}

struct EmptyParkourProvider;

impl ServerChunkProvider for EmptyParkourProvider {
    fn generate(&self, request: &ChunkGenerationRequest) -> Option<Chunk> {
        Some(Chunk::filled(request.position, BlockId::Air))
    }
}
