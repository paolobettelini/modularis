use bevy::prelude::Resource;
use serde::{Serialize, de::DeserializeOwned};
use std::{any::Any, collections::HashMap, error::Error, fmt, hash::Hash};

pub trait BlockComponent: Serialize + DeserializeOwned + Send + Sync + 'static {
    const ID: &'static str;
    const VERSION: u32;

    fn encode(&self) -> Result<Vec<u8>, BlockComponentError> {
        serde_cbor::to_vec(self).map_err(|error| BlockComponentError(error.to_string()))
    }

    fn decode_version(version: u32, payload: &[u8]) -> Result<Self, BlockComponentError> where Self: Sized {
        if version != Self::VERSION {
            return Err(BlockComponentError(format!(
                "component '{}' cannot decode version {version}; current version is {}",
                Self::ID, Self::VERSION,
            )));
        }
        serde_cbor::from_slice(payload).map_err(|error| BlockComponentError(error.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockComponentError(pub String);

impl fmt::Display for BlockComponentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(formatter) }
}

impl Error for BlockComponentError {}

trait ErasedValue: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any + Send + Sync> ErasedValue for T {
    fn as_any(&self) -> &dyn Any { self }
}

struct Codec {
    version: u32,
    encode: fn(&dyn ErasedValue) -> Result<Vec<u8>, BlockComponentError>,
    decode: fn(u32, &[u8]) -> Result<Box<dyn ErasedValue>, BlockComponentError>,
}

#[derive(Resource, Default)]
pub struct BlockComponentRegistry {
    codecs: HashMap<String, Codec>,
}

impl BlockComponentRegistry {
    pub fn register<T: BlockComponent>(&mut self) {
        assert!(T::ID.contains(':'), "block component IDs must be namespaced");
        let codec = Codec {
            version: T::VERSION,
            encode: |value| {
                let value = value.as_any().downcast_ref::<T>()
                    .ok_or_else(|| BlockComponentError(format!("type mismatch for component '{}'", T::ID)))?;
                value.encode()
            },
            decode: |version, payload| Ok(Box::new(T::decode_version(version, payload)?)),
        };
        assert!(self.codecs.insert(T::ID.to_string(), codec).is_none(), "duplicate block component codec '{}'", T::ID);
    }

    pub fn is_registered(&self, id: &str) -> bool { self.codecs.contains_key(id) }
}

enum StoredValue {
    Decoded(Box<dyn ErasedValue>),
    Opaque { version: u32, payload: Vec<u8> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodedBlockComponent {
    pub local_index: u16,
    pub component_id: String,
    pub version: u32,
    pub payload: Vec<u8>,
}

/// Sparse by chunk key, then local block index, then stable component ID.
/// A chunk without components has no entry and allocates no 4096-cell array.
pub struct SparseBlockComponents<K> {
    chunks: HashMap<K, HashMap<u16, HashMap<String, StoredValue>>>,
}

impl<K> Default for SparseBlockComponents<K> {
    fn default() -> Self { Self { chunks: HashMap::new() } }
}

impl<K: Eq + Hash> SparseBlockComponents<K> {
    pub fn get<T: BlockComponent>(&self, chunk: &K, local_index: u16) -> Option<&T> {
        let StoredValue::Decoded(value) = self.chunks.get(chunk)?.get(&local_index)?.get(T::ID)? else { return None };
        // Dispatch `as_any` through the erased value. Calling it directly on
        // `Box<dyn ErasedValue>` selects the blanket implementation for Box
        // itself, making every freshly inserted component fail its downcast.
        value.as_ref().as_any().downcast_ref::<T>()
    }

    pub fn entries<T: BlockComponent>(&self, chunk: &K) -> Vec<(u16, &T)> {
        self.chunks.get(chunk).into_iter().flat_map(|positions| positions.iter()).filter_map(|(index, components)| {
            let StoredValue::Decoded(value) = components.get(T::ID)? else { return None };
            Some((*index, value.as_ref().as_any().downcast_ref::<T>()?))
        }).collect()
    }

    pub fn set<T: BlockComponent>(&mut self, chunk: K, local_index: u16, value: T) {
        self.chunks.entry(chunk).or_default().entry(local_index).or_default()
            .insert(T::ID.to_string(), StoredValue::Decoded(Box::new(value)));
    }

    pub fn remove<T: BlockComponent>(&mut self, chunk: &K, local_index: u16) -> bool {
        self.remove_by_id(chunk, local_index, T::ID)
    }

    pub fn remove_all_at(&mut self, chunk: &K, local_index: u16) -> bool {
        let Some(positions) = self.chunks.get_mut(chunk) else { return false };
        let removed = positions.remove(&local_index).is_some();
        if positions.is_empty() { self.chunks.remove(chunk); }
        removed
    }

    pub fn chunk_is_empty(&self, chunk: &K) -> bool { !self.chunks.contains_key(chunk) }

    pub fn remove_chunk(&mut self, chunk: &K) -> bool {
        self.chunks.remove(chunk).is_some()
    }

    pub fn encode_chunk(&self, chunk: &K, registry: &BlockComponentRegistry) -> Result<Vec<EncodedBlockComponent>, BlockComponentError> {
        let Some(positions) = self.chunks.get(chunk) else { return Ok(Vec::new()) };
        let mut records = Vec::new();
        for (local_index, components) in positions {
            for (id, value) in components {
                let (version, payload) = match value {
                    StoredValue::Decoded(value) => {
                        let codec = registry.codecs.get(id).ok_or_else(|| BlockComponentError(format!("missing codec for component '{id}'")))?;
                        (codec.version, (codec.encode)(value.as_ref())?)
                    }
                    StoredValue::Opaque { version, payload } => (*version, payload.clone()),
                };
                records.push(EncodedBlockComponent { local_index: *local_index, component_id: id.clone(), version, payload });
            }
        }
        records.sort_by(|a, b| (a.local_index, &a.component_id).cmp(&(b.local_index, &b.component_id)));
        Ok(records)
    }

    pub fn replace_chunk(&mut self, chunk: K, records: Vec<EncodedBlockComponent>, registry: &BlockComponentRegistry) -> Result<(), BlockComponentError> {
        let mut positions = HashMap::<u16, HashMap<String, StoredValue>>::new();
        for record in records {
            let value = match registry.codecs.get(&record.component_id) {
                Some(codec) => match (codec.decode)(record.version, &record.payload) {
                    Ok(value) => StoredValue::Decoded(value),
                    Err(_) => StoredValue::Opaque { version: record.version, payload: record.payload },
                },
                None => StoredValue::Opaque { version: record.version, payload: record.payload },
            };
            if positions.entry(record.local_index).or_default().insert(record.component_id.clone(), value).is_some() {
                return Err(BlockComponentError(format!("duplicate component '{}' at local index {}", record.component_id, record.local_index)));
            }
        }
        if positions.is_empty() { self.chunks.remove(&chunk); } else { self.chunks.insert(chunk, positions); }
        Ok(())
    }

    fn remove_by_id(&mut self, chunk: &K, local_index: u16, id: &str) -> bool {
        let Some(positions) = self.chunks.get_mut(chunk) else { return false };
        let Some(components) = positions.get_mut(&local_index) else { return false };
        let removed = components.remove(id).is_some();
        if components.is_empty() { positions.remove(&local_index); }
        if positions.is_empty() { self.chunks.remove(chunk); }
        removed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct TestComponent(u32);

    impl BlockComponent for TestComponent {
        const ID: &'static str = "test:value";
        const VERSION: u32 = 1;
    }

    #[test]
    fn freshly_inserted_component_can_be_read_through_erasure() {
        let mut components = SparseBlockComponents::<u64>::default();
        components.set(7, 12, TestComponent(41));

        assert_eq!(components.get::<TestComponent>(&7, 12), Some(&TestComponent(41)));
        let entries = components.entries::<TestComponent>(&7);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].0, 12);
        assert_eq!(entries[0].1, &TestComponent(41));
    }
}
