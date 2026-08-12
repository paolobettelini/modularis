use block_component_api::{BlockComponentError, BlockComponentRegistry};
use server_block_component_api::ServerBlockComponents;
use server_chunk_world_api::ResidentChunkKey;
use server_world_data_storage_api::{ServerWorldDataStorage, WorldDataKey};

pub const BLOCK_COMPONENT_DOMAIN: &str = "modularis:block-components";

pub fn storage_key(key: &ResidentChunkKey) -> WorldDataKey {
    WorldDataKey {
        instance: key.instance.clone(),
        domain: BLOCK_COMPONENT_DOMAIN.to_string(),
        source: key.provider.0.clone(),
        partition: key.position,
    }
}

pub fn load_if_needed(
    components: &mut ServerBlockComponents,
    key: &ResidentChunkKey,
    registry: &BlockComponentRegistry,
    storage: &ServerWorldDataStorage,
) -> Result<(), BlockComponentError> {
    if components.is_loaded(key) { return Ok(()); }
    let payload = storage.load(&storage_key(key)).map_err(|error| BlockComponentError(error.to_string()))?;
    let records = payload.map(|payload| block_component_binary_format_lib::decode(&payload)).transpose()?.unwrap_or_default();
    components.replace_loaded_chunk(key.clone(), records, registry)
}

pub fn queue_dirty(
    components: &mut ServerBlockComponents,
    registry: &BlockComponentRegistry,
    storage: &ServerWorldDataStorage,
) -> Result<usize, BlockComponentError> {
    let mut persisted = 0;
    for key in components.dirty_keys() {
        let records = components.encoded_chunk(&key, registry)?;
        let payload = (!records.is_empty()).then(|| block_component_binary_format_lib::encode(&records)).transpose()?;
        let queued = storage.queue_store(&storage_key(&key), payload.as_deref())
            .map_err(|error| BlockComponentError(error.to_string()))?;
        if queued { components.mark_persisted(&key); persisted += 1; }
    }
    Ok(persisted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use block_component_api::BlockComponentRegistry;
    use block_durability_api::BlockDamage;
    use server_chunk_provider_api::ChunkProviderId;
    use voxel_math_api::ChunkPos;
    use world_instance_api::WorldInstanceId;

    fn key() -> ResidentChunkKey {
        ResidentChunkKey { instance: WorldInstanceId::new("test:world"), provider: ChunkProviderId::new("test:provider"), position: ChunkPos::new(1, -2, 3) }
    }

    #[test]
    fn saves_and_loads_sparse_components_through_the_storage_boundary() {
        let mut registry = BlockComponentRegistry::default();
        registry.register::<BlockDamage>();
        let storage = ServerWorldDataStorage::memory();
        let mut source = ServerBlockComponents::default();
        source.set(key(), 42, BlockDamage(17));
        assert_eq!(queue_dirty(&mut source, &registry, &storage).unwrap(), 1);

        let mut restored = ServerBlockComponents::default();
        load_if_needed(&mut restored, &key(), &registry, &storage).unwrap();
        assert_eq!(restored.get::<BlockDamage>(&key(), 42), Some(&BlockDamage(17)));
    }
}
