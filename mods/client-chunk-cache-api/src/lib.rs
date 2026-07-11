use bevy::prelude::*;
use block_instance_api::BlockInstance;
use chunk_api::Chunk;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use voxel_math_api::{BlockPos, ChunkPos};

#[derive(Resource, Clone, Default)]
pub struct ClientChunkCache {
    chunks: Arc<RwLock<HashMap<ChunkPos, Chunk>>>,
}

impl ClientChunkCache {
    pub fn insert(&self, chunk: Chunk) {
        self.chunks
            .write()
            .expect("client chunk cache lock poisoned")
            .insert(chunk.position(), chunk);
    }

    pub fn remove(&self, position: ChunkPos) {
        self.chunks
            .write()
            .expect("client chunk cache lock poisoned")
            .remove(&position);
    }

    pub fn clear(&self) {
        self.chunks
            .write()
            .expect("client chunk cache lock poisoned")
            .clear();
    }

    pub fn chunk(&self, position: ChunkPos) -> Option<Chunk> {
        self.chunks
            .read()
            .expect("client chunk cache lock poisoned")
            .get(&position)
            .cloned()
    }

    pub fn block(&self, position: BlockPos) -> Option<BlockInstance> {
        self.chunks
            .read()
            .expect("client chunk cache lock poisoned")
            .get(&position.chunk())
            .map(|chunk| chunk.get(position.local()))
    }

    pub fn set_block(&self, position: BlockPos, block: impl Into<BlockInstance>) -> bool {
        let mut chunks = self
            .chunks
            .write()
            .expect("client chunk cache lock poisoned");
        let Some(chunk) = chunks.get_mut(&position.chunk()) else {
            return false;
        };
        chunk.set(position.local(), block.into());
        true
    }
}

#[derive(Message, Debug, Clone, Copy)]
pub struct ClientChunkAvailable {
    pub position: ChunkPos,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct ClientChunkChanged {
    pub position: ChunkPos,
}

pub trait ClientChunkCacheApi: Send + Sync + 'static {}
