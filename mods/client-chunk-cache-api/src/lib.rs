use bevy::prelude::*;
use block_state_api::BlockState;
use chunk_api::Chunk;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};
use voxel_frame_api::{VoxelBlockAddress, VoxelChunkAddress, VoxelFrameId};

#[derive(Resource, Clone, Default)]
pub struct ClientChunkCache {
    chunks: Arc<RwLock<HashMap<VoxelChunkAddress, Chunk>>>,
}

impl ClientChunkCache {
    pub fn is_empty(&self) -> bool {
        self.chunks
            .read()
            .expect("client chunk cache lock poisoned")
            .is_empty()
    }

    pub fn insert(&self, chunk: Chunk) { self.insert_in_frame(VoxelFrameId::ROOT,chunk); }

    pub fn insert_in_frame(&self, frame: VoxelFrameId, chunk: Chunk) {
        self.chunks
            .write()
            .expect("client chunk cache lock poisoned")
            .insert(VoxelChunkAddress::new(frame,chunk.position()), chunk);
    }

    pub fn remove(&self, position: impl Into<VoxelChunkAddress>) {
        let position = position.into();
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

    pub fn chunk(&self, position: impl Into<VoxelChunkAddress>) -> Option<Chunk> {
        let position = position.into();
        self.chunks
            .read()
            .expect("client chunk cache lock poisoned")
            .get(&position)
            .cloned()
    }

    pub fn contains(&self, position: impl Into<VoxelChunkAddress>) -> bool {
        let position = position.into();
        self.chunks
            .read()
            .expect("client chunk cache lock poisoned")
            .contains_key(&position)
    }

    pub fn missing_from(&self, positions: &HashSet<VoxelChunkAddress>) -> Vec<VoxelChunkAddress> {
        let chunks = self
            .chunks
            .read()
            .expect("client chunk cache lock poisoned");
        positions
            .iter()
            .filter(|position| !chunks.contains_key(position))
            .copied()
            .collect()
    }

    pub fn uniform_block(&self, position: impl Into<VoxelChunkAddress>) -> Option<BlockState> {
        let position = position.into();
        self.chunks
            .read()
            .expect("client chunk cache lock poisoned")
            .get(&position)
            .and_then(Chunk::uniform_block)
    }

    pub fn block(&self, position: impl Into<VoxelBlockAddress>) -> Option<BlockState> {
        let position = position.into();
        self.chunks
            .read()
            .expect("client chunk cache lock poisoned")
            .get(&position.chunk())
            .map(|chunk| chunk.get(position.local()))
    }

    pub fn set_block(&self, position: impl Into<VoxelBlockAddress>, block: impl Into<BlockState>) -> bool {
        let position = position.into();
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
    pub position: VoxelChunkAddress,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct ClientChunkChanged {
    pub position: VoxelChunkAddress,
}

pub trait ClientChunkCacheApi: Send + Sync + 'static {}
