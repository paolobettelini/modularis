use bevy_mod::BevyMod;
use chunk_api::Chunk;
use generated_block_registry::BlockId;
use server_chunk_provider_api::{
    ChunkGenerationRequest, ChunkProviderId, ServerChunkProvider, ServerChunkProviderRegistry,
};
use server_chunk_provider_registry_mod::ServerChunkProviderRegistryMod;
use server_primary_chunk_provider_api::ServerPrimaryChunkProviderApi;
use thecrown_world_template_api::{TheCrownWorldTemplateApi, TheCrownWorldTemplates};
use tokio::task::JoinHandle;

pub struct ServerChunkProviderTheCrownMod;

impl ServerChunkProviderTheCrownMod {
    pub fn init<T: TheCrownWorldTemplateApi>(
        bevy: &mut BevyMod,
        _registry: &mut ServerChunkProviderRegistryMod,
        _templates_api: &mut T,
    ) -> Self {
        let templates = bevy.app.world().resource::<TheCrownWorldTemplates>().clone();
        bevy.app.world().resource::<ServerChunkProviderRegistry>()
            .register(ChunkProviderId::primary(), TheCrownProvider { templates })
            .expect("the primary server chunk provider must be unique");
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl ServerPrimaryChunkProviderApi for ServerChunkProviderTheCrownMod {}

struct TheCrownProvider { templates: TheCrownWorldTemplates }

impl ServerChunkProvider for TheCrownProvider {
    fn generate(&self, request: &ChunkGenerationRequest) -> Option<Chunk> {
        Some(self.templates.chunk(&request.instance, request.position)
            .unwrap_or_else(|| Chunk::filled(request.position, BlockId::Air)))
    }
}
