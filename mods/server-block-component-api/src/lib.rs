use bevy::prelude::*;
use block_component_api::{
    BlockComponent, BlockComponentError, BlockComponentRegistry, EncodedBlockComponent,
    SparseBlockComponents,
};
use server_chunk_world_api::ResidentChunkKey;
use std::collections::HashSet;

#[derive(Resource, Default)]
pub struct ServerBlockComponents {
    values: SparseBlockComponents<ResidentChunkKey>,
    loaded: HashSet<ResidentChunkKey>,
    dirty: HashSet<ResidentChunkKey>,
}

impl ServerBlockComponents {
    pub fn is_loaded(&self, key: &ResidentChunkKey) -> bool { self.loaded.contains(key) }

    pub fn get<T: BlockComponent>(&self, key: &ResidentChunkKey, local_index: u16) -> Option<&T> {
        self.values.get::<T>(key, local_index)
    }

    pub fn entries<T: BlockComponent>(&self, key: &ResidentChunkKey) -> Vec<(u16, &T)> {
        self.values.entries::<T>(key)
    }

    pub fn set<T: BlockComponent>(&mut self, key: ResidentChunkKey, local_index: u16, value: T) {
        self.values.set(key.clone(), local_index, value);
        self.loaded.insert(key.clone());
        self.dirty.insert(key);
    }

    pub fn remove<T: BlockComponent>(&mut self, key: &ResidentChunkKey, local_index: u16) -> bool {
        let removed = self.values.remove::<T>(key, local_index);
        if removed { self.dirty.insert(key.clone()); }
        removed
    }

    pub fn remove_all_at(&mut self, key: &ResidentChunkKey, local_index: u16) -> bool {
        let removed = self.values.remove_all_at(key, local_index);
        if removed { self.dirty.insert(key.clone()); }
        removed
    }

    pub fn replace_loaded_chunk(
        &mut self,
        key: ResidentChunkKey,
        records: Vec<EncodedBlockComponent>,
        registry: &BlockComponentRegistry,
    ) -> Result<(), BlockComponentError> {
        self.values.replace_chunk(key.clone(), records, registry)?;
        self.loaded.insert(key);
        Ok(())
    }

    pub fn encoded_chunk(
        &self,
        key: &ResidentChunkKey,
        registry: &BlockComponentRegistry,
    ) -> Result<Vec<EncodedBlockComponent>, BlockComponentError> {
        self.values.encode_chunk(key, registry)
    }

    pub fn dirty_keys(&self) -> Vec<ResidentChunkKey> { self.dirty.iter().cloned().collect() }
    pub fn mark_persisted(&mut self, key: &ResidentChunkKey) { self.dirty.remove(key); }

    pub fn unload_clean_except(&mut self, retained: &HashSet<ResidentChunkKey>) {
        let removable = self.loaded.iter()
            .filter(|key| !retained.contains(*key) && !self.dirty.contains(*key))
            .cloned()
            .collect::<Vec<_>>();
        for key in removable {
            self.values.remove_chunk(&key);
            self.loaded.remove(&key);
        }
    }
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerBlockComponentSet { Load, Mutate, Persist }

pub trait ServerBlockComponentApi: Send + Sync + 'static {}
