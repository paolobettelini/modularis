use bevy::prelude::*;
use chunk_api::Chunk;
use std::{
    collections::HashMap,
    error::Error,
    fmt,
    sync::{Arc, RwLock},
    time::Duration,
};
use voxel_math_api::ChunkPos;
use world_instance_api::WorldInstanceId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StoredChunkKey {
    pub instance: WorldInstanceId,
    pub source: String,
    pub position: ChunkPos,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ChunkStorageFlushReport {
    pub regions_written: usize,
    pub chunks_written: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkStorageError(pub String);

impl fmt::Display for ChunkStorageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl Error for ChunkStorageError {}

pub trait ServerChunkStorageBackend: Send + Sync + 'static {
    fn load(&self, key: &StoredChunkKey) -> Result<Option<Chunk>, ChunkStorageError>;

    /// Returns false when the world is intentionally outside this backend's
    /// catalog and therefore remains transient.
    fn queue_store(&self, key: &StoredChunkKey, chunk: &Chunk) -> Result<bool, ChunkStorageError>;

    fn flush(&self) -> Result<ChunkStorageFlushReport, ChunkStorageError>;

    fn pending_chunks(&self) -> usize;
}

#[derive(Resource, Clone)]
pub struct ServerChunkStorage(Arc<dyn ServerChunkStorageBackend>);

impl ServerChunkStorage {
    pub fn new<B: ServerChunkStorageBackend>(backend: B) -> Self {
        Self(Arc::new(backend))
    }

    pub fn memory() -> Self {
        Self::new(MemoryChunkStorage::default())
    }

    pub fn load(&self, key: &StoredChunkKey) -> Result<Option<Chunk>, ChunkStorageError> {
        self.0.load(key)
    }

    pub fn queue_store(
        &self,
        key: &StoredChunkKey,
        chunk: &Chunk,
    ) -> Result<bool, ChunkStorageError> {
        self.0.queue_store(key, chunk)
    }

    pub fn flush(&self) -> Result<ChunkStorageFlushReport, ChunkStorageError> {
        self.0.flush()
    }

    pub fn pending_chunks(&self) -> usize {
        self.0.pending_chunks()
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct ChunkStorageFlushInterval(pub Duration);

impl Default for ChunkStorageFlushInterval {
    fn default() -> Self {
        Self(Duration::from_secs(5))
    }
}

pub trait ServerChunkStorageApi: Send + Sync + 'static {}

#[derive(Default)]
struct MemoryChunkStorage {
    chunks: RwLock<HashMap<StoredChunkKey, Chunk>>,
}

impl ServerChunkStorageBackend for MemoryChunkStorage {
    fn load(&self, key: &StoredChunkKey) -> Result<Option<Chunk>, ChunkStorageError> {
        Ok(self
            .chunks
            .read()
            .expect("memory chunk storage lock poisoned")
            .get(key)
            .cloned())
    }

    fn queue_store(&self, key: &StoredChunkKey, chunk: &Chunk) -> Result<bool, ChunkStorageError> {
        self.chunks
            .write()
            .expect("memory chunk storage lock poisoned")
            .insert(key.clone(), chunk.clone());
        Ok(true)
    }

    fn flush(&self) -> Result<ChunkStorageFlushReport, ChunkStorageError> {
        Ok(ChunkStorageFlushReport::default())
    }

    fn pending_chunks(&self) -> usize {
        0
    }
}
