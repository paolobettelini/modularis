use bevy::prelude::*;
use chunk_api::Chunk;
use player_network_message_types::PlayerId;
use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, RwLock},
};
use voxel_math_api::ChunkPos;
use world_instance_api::WorldInstanceId;

pub const PRIMARY_CHUNK_PROVIDER_ID: &str = "patchwork:primary";

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChunkProviderId(pub String);

impl ChunkProviderId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn primary() -> Self {
        Self::new(PRIMARY_CHUNK_PROVIDER_ID)
    }
}

impl fmt::Display for ChunkProviderId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChunkViewer {
    Server,
    Player(PlayerId),
}

#[derive(Debug, Clone)]
pub struct ChunkGenerationRequest {
    pub viewer: ChunkViewer,
    pub instance: WorldInstanceId,
    pub position: ChunkPos,
}

pub trait ServerChunkProvider: Send + Sync + 'static {
    fn generate(&self, request: &ChunkGenerationRequest) -> Option<Chunk>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChunkProviderRegistryError {
    DuplicateProvider(ChunkProviderId),
}

#[derive(Resource, Clone, Default)]
pub struct ServerChunkProviderRegistry {
    providers: Arc<RwLock<HashMap<ChunkProviderId, Arc<dyn ServerChunkProvider>>>>,
}

impl ServerChunkProviderRegistry {
    pub fn register(
        &self,
        id: ChunkProviderId,
        provider: impl ServerChunkProvider,
    ) -> Result<(), ChunkProviderRegistryError> {
        let mut providers = self
            .providers
            .write()
            .expect("server chunk provider registry lock poisoned");
        if providers.contains_key(&id) {
            return Err(ChunkProviderRegistryError::DuplicateProvider(id));
        }
        providers.insert(id, Arc::new(provider));
        Ok(())
    }

    pub fn generate(
        &self,
        provider: &ChunkProviderId,
        request: &ChunkGenerationRequest,
    ) -> Option<Chunk> {
        self.providers
            .read()
            .expect("server chunk provider registry lock poisoned")
            .get(provider)
            .cloned()
            .and_then(|provider| provider.generate(request))
    }

    pub fn contains(&self, provider: &ChunkProviderId) -> bool {
        self.providers
            .read()
            .expect("server chunk provider registry lock poisoned")
            .contains_key(provider)
    }
}
