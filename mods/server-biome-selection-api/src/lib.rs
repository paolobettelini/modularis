use bevy::prelude::Resource;
use server_biome_api::{BiomeDefinition, BiomeId};
use server_chunk_provider_api::ChunkGenerationRequest;
use std::sync::Arc;

pub struct BiomeSelectionRequest<'a> {
    pub generation: &'a ChunkGenerationRequest,
    pub x: i32,
    pub z: i32,
}

pub trait ServerBiomeSelector: Send + Sync + 'static {
    fn select(
        &self,
        request: &BiomeSelectionRequest<'_>,
        definitions: &[BiomeDefinition],
    ) -> Option<BiomeId>;
}

#[derive(Resource, Clone)]
pub struct ServerBiomeSelectorResource(Arc<dyn ServerBiomeSelector>);

impl ServerBiomeSelectorResource {
    pub fn new(selector: impl ServerBiomeSelector) -> Self {
        Self(Arc::new(selector))
    }

    pub fn select(
        &self,
        request: &BiomeSelectionRequest<'_>,
        definitions: &[BiomeDefinition],
    ) -> Option<BiomeId> {
        self.0.select(request, definitions)
    }
}

pub trait ServerBiomeSelectionApi: Send + Sync + 'static {}
