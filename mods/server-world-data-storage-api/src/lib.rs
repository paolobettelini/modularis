use bevy::prelude::*;
use std::{collections::HashMap, error::Error, fmt, sync::{Arc, RwLock}, time::Duration};
use voxel_math_api::ChunkPos;
use world_instance_api::WorldInstanceId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorldDataKey {
    pub instance: WorldInstanceId,
    /// Stable, namespaced persistence domain. Each world system owns its domain.
    pub domain: String,
    /// Provider/source scope inside the world.
    pub source: String,
    /// Domain-specific spatial partition. Block components use chunk position.
    pub partition: ChunkPos,
}

impl WorldDataKey {
    pub fn validate(&self) -> Result<(), WorldDataStorageError> {
        if !self.domain.contains(':') || self.domain.chars().any(char::is_whitespace) {
            return Err(WorldDataStorageError(format!("world data domain '{}' must be namespaced", self.domain)));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WorldDataFlushReport { pub records_written: usize, pub records_deleted: usize }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldDataStorageError(pub String);
impl fmt::Display for WorldDataStorageError { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) } }
impl Error for WorldDataStorageError {}

pub trait ServerWorldDataStorageBackend: Send + Sync + 'static {
    fn load(&self, key: &WorldDataKey) -> Result<Option<Vec<u8>>, WorldDataStorageError>;
    fn queue_store(&self, key: &WorldDataKey, payload: Option<&[u8]>) -> Result<bool, WorldDataStorageError>;
    fn flush(&self) -> Result<WorldDataFlushReport, WorldDataStorageError>;
    fn pending_records(&self) -> usize;
}

#[derive(Resource, Clone)]
pub struct ServerWorldDataStorage(Arc<dyn ServerWorldDataStorageBackend>);

impl ServerWorldDataStorage {
    pub fn new(backend: impl ServerWorldDataStorageBackend) -> Self { Self(Arc::new(backend)) }
    pub fn memory() -> Self { Self::new(MemoryWorldDataStorage::default()) }
    pub fn load(&self, key: &WorldDataKey) -> Result<Option<Vec<u8>>, WorldDataStorageError> { key.validate()?; self.0.load(key) }
    pub fn queue_store(&self, key: &WorldDataKey, payload: Option<&[u8]>) -> Result<bool, WorldDataStorageError> { key.validate()?; self.0.queue_store(key, payload) }
    pub fn flush(&self) -> Result<WorldDataFlushReport, WorldDataStorageError> { self.0.flush() }
    pub fn pending_records(&self) -> usize { self.0.pending_records() }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct WorldDataFlushInterval(pub Duration);
impl Default for WorldDataFlushInterval { fn default() -> Self { Self(Duration::from_secs(5)) } }

pub trait ServerWorldDataStorageApi: Send + Sync + 'static {}

#[derive(Default)]
struct MemoryWorldDataStorage { data: RwLock<HashMap<WorldDataKey, Vec<u8>>> }
impl ServerWorldDataStorageBackend for MemoryWorldDataStorage {
    fn load(&self, key: &WorldDataKey) -> Result<Option<Vec<u8>>, WorldDataStorageError> { Ok(self.data.read().unwrap().get(key).cloned()) }
    fn queue_store(&self, key: &WorldDataKey, payload: Option<&[u8]>) -> Result<bool, WorldDataStorageError> {
        let mut data = self.data.write().unwrap();
        match payload { Some(payload) => { data.insert(key.clone(), payload.to_vec()); }, None => { data.remove(key); } }
        Ok(true)
    }
    fn flush(&self) -> Result<WorldDataFlushReport, WorldDataStorageError> { Ok(WorldDataFlushReport::default()) }
    fn pending_records(&self) -> usize { 0 }
}

