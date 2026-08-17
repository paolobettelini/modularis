use bevy::prelude::*;
use chunk_api::Chunk;
use std::{collections::HashMap, sync::{Arc, RwLock}};
use voxel_math_api::ChunkPos;
use world_instance_api::WorldInstanceId;

type WorldTemplate = HashMap<ChunkPos, Chunk>;

#[derive(Resource, Clone, Default)]
pub struct TheCrownWorldTemplates(Arc<RwLock<HashMap<WorldInstanceId, WorldTemplate>>>);

impl TheCrownWorldTemplates {
    pub fn install_chunks(&self, instance: WorldInstanceId, chunks: impl IntoIterator<Item = Chunk>) {
        let chunks = chunks.into_iter().map(|chunk| (chunk.position(), chunk)).collect();
        self.0.write().expect("TheCrown world templates lock poisoned").insert(instance, chunks);
    }

    pub fn chunk(&self, instance: &WorldInstanceId, position: ChunkPos) -> Option<Chunk> {
        self.0.read().expect("TheCrown world templates lock poisoned")
            .get(instance).and_then(|chunks| chunks.get(&position)).cloned()
    }

    pub fn remove(&self, instance: &WorldInstanceId) {
        self.0.write().expect("TheCrown world templates lock poisoned").remove(instance);
    }
}

pub trait TheCrownWorldTemplateApi: Send + Sync + 'static {}
