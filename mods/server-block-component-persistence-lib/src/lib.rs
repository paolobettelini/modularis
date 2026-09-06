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
        frame: key.frame,
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
        ResidentChunkKey { frame: voxel_frame_api::VoxelFrameId::ROOT, instance: WorldInstanceId::new("test:world"), provider: ChunkProviderId::new("test:provider"), position: ChunkPos::new(1, -2, 3) }
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
    #[test]
    fn save_load_separates_frames_at_the_same_chunk_and_local_index() {
        let mut registry=BlockComponentRegistry::default();registry.register::<BlockDamage>();
        let storage=ServerWorldDataStorage::memory();
        let mut first=key();first.frame=voxel_frame_api::VoxelFrameId::new();
        let mut second=first.clone();second.frame=voxel_frame_api::VoxelFrameId::new();
        let mut data=ServerBlockComponents::default();
        data.set(first.clone(),42,BlockDamage(12));data.set(second.clone(),42,BlockDamage(31));
        queue_dirty(&mut data,&registry,&storage).unwrap();
        let mut loaded=ServerBlockComponents::default();
        load_if_needed(&mut loaded,&first,&registry,&storage).unwrap();
        load_if_needed(&mut loaded,&second,&registry,&storage).unwrap();
        assert_eq!(loaded.get::<BlockDamage>(&first,42),Some(&BlockDamage(12)));
        assert_eq!(loaded.get::<BlockDamage>(&second,42),Some(&BlockDamage(31)));
    }

}
