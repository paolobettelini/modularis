use block_durability_api::{BlockDamage, BlockDurability, BlockDurabilityProperty};
use block_properties_api::BlockProperties;
use block_state_api::BlockId;
use server_block_component_api::ServerBlockComponents;
use server_chunk_world_api::ResidentChunkKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockDurabilityStatus {
    pub default: BlockDurability,
    pub damage: u32,
    pub remaining: Option<u32>,
    pub broken: bool,
}

pub fn status(
    properties: &BlockProperties,
    components: &ServerBlockComponents,
    key: &ResidentChunkKey,
    local_index: u16,
    block: BlockId,
) -> BlockDurabilityStatus {
    let default = properties.get::<BlockDurabilityProperty>(block);
    let stored_damage = components.get::<BlockDamage>(key, local_index).map(|damage| damage.0).unwrap_or(0);
    match default {
        BlockDurability::Breakable(points) => {
            BlockDurabilityStatus {
                default,
                damage: stored_damage,
                remaining: Some(points.saturating_sub(stored_damage)),
                broken: stored_damage >= points,
            }
        }
        BlockDurability::Unbreakable => BlockDurabilityStatus {
            default,
            damage: 0,
            remaining: None,
            broken: false,
        },
    }
}

pub fn apply_damage(
    properties: &BlockProperties,
    components: &mut ServerBlockComponents,
    key: ResidentChunkKey,
    local_index: u16,
    block: BlockId,
    amount: u32,
) -> BlockDurabilityStatus {
    let current = status(properties, components, &key, local_index, block);
    if current.broken || !matches!(current.default, BlockDurability::Breakable(_)) {
        components.remove::<BlockDamage>(&key, local_index);
        return current;
    }
    let damage = current.damage.saturating_add(amount);
    if damage == 0 { components.remove::<BlockDamage>(&key, local_index); }
    else { components.set(key.clone(), local_index, BlockDamage(damage)); }
    status(properties, components, &key, local_index, block)
}

pub fn restore_default(components: &mut ServerBlockComponents, key: &ResidentChunkKey, local_index: u16) -> bool {
    components.remove::<BlockDamage>(key, local_index)
}

pub fn discrete_stage(status: BlockDurabilityStatus, stages: u8) -> u8 {
    if status.damage == 0 || stages == 0 { return 0; }
    let BlockDurability::Breakable(default) = status.default else { return 0 };
    if default == 0 { return 0; }
    let progressed = (u64::from(status.damage.min(default)) * u64::from(stages) + u64::from(default) - 1) / u64::from(default);
    progressed.clamp(1, u64::from(stages)) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use block_component_api::{BlockComponentRegistry, SparseBlockComponents};
    use block_durability_api::{BlockDamage, BlockDurability, COMMON_BLOCK_DURABILITY};
    use block_state_api::BlockId;
    use chunk_section_api::ChunkSection;
    use server_chunk_provider_api::ChunkProviderId;
    use voxel_math_api::ChunkPos;
    use world_instance_api::WorldInstanceId;

    fn key() -> ResidentChunkKey {
        ResidentChunkKey { frame: voxel_frame_api::VoxelFrameId::ROOT, instance: WorldInstanceId::new("test:world"), provider: ChunkProviderId::new("test:terrain"), position: ChunkPos::new(0, 0, 0) }
    }

    #[test]
    fn default_override_sparse_removal_and_palette_separation() {
        let mut properties = BlockProperties::default();
        let mut components = ServerBlockComponents::default();
        let key = key();
        assert_eq!(properties.get::<BlockDurabilityProperty>(BlockId::Stone), COMMON_BLOCK_DURABILITY);
        properties.set::<BlockDurabilityProperty>(BlockId::Stone, BlockDurability::breakable(250));
        assert_eq!(status(&properties, &components, &key, 0, BlockId::Stone).default, BlockDurability::breakable(250));
        assert!(components.get::<BlockDamage>(&key, 0).is_none());
        assert_eq!(apply_damage(&properties, &mut components, key.clone(), 0, BlockId::Stone, 30).damage, 30);
        assert!(restore_default(&mut components, &key, 0));
        assert!(components.get::<BlockDamage>(&key, 0).is_none());

        let palette_before = ChunkSection::filled(BlockId::Stone);
        apply_damage(&properties, &mut components, key.clone(), 1, BlockId::Stone, 12);
        assert_eq!(palette_before.palette().len(), 1);
        assert_eq!(palette_before.palette()[0].block, BlockId::Stone);
    }

    #[test]
    fn instant_and_unbreakable_blocks_never_allocate_damage() {
        let mut properties = BlockProperties::default();
        let mut components = ServerBlockComponents::default();
        let key = key();
        properties.set::<BlockDurabilityProperty>(BlockId::Dirt, BlockDurability::Breakable(0));
        properties.set::<BlockDurabilityProperty>(BlockId::Bedrock, BlockDurability::Unbreakable);

        assert!(apply_damage(&properties, &mut components, key.clone(), 3, BlockId::Dirt, 1).broken);
        assert!(!apply_damage(&properties, &mut components, key.clone(), 4, BlockId::Bedrock, u32::MAX).broken);
        assert!(components.get::<BlockDamage>(&key, 3).is_none());
        assert!(components.get::<BlockDamage>(&key, 4).is_none());
    }

    #[test]
    fn damage_survives_versioned_binary_save_and_load() {
        let mut registry = BlockComponentRegistry::default();
        registry.register::<BlockDamage>();
        let mut source = SparseBlockComponents::default();
        source.set(key(), 7, BlockDamage(41));
        let records = source.encode_chunk(&key(), &registry).unwrap();
        let bytes = block_component_binary_format_lib::encode(&records).unwrap();
        let mut restored = SparseBlockComponents::default();
        restored.replace_chunk(key(), block_component_binary_format_lib::decode(&bytes).unwrap(), &registry).unwrap();
        assert_eq!(restored.get::<BlockDamage>(&key(), 7), Some(&BlockDamage(41)));
    }
    #[test]
    fn damage_at_the_same_local_position_is_isolated_between_frames() {
        let properties=BlockProperties::default();
        let mut data=ServerBlockComponents::default();
        let mut first=key();first.frame=voxel_frame_api::VoxelFrameId::new();
        let mut second=first.clone();second.frame=voxel_frame_api::VoxelFrameId::new();
        apply_damage(&properties,&mut data,first.clone(),42,BlockId::Stone,12);
        assert_eq!(status(&properties,&data,&first,42,BlockId::Stone).damage,12);
        assert_eq!(status(&properties,&data,&second,42,BlockId::Stone).damage,0);
        assert!(data.get::<BlockDamage>(&second,42).is_none());
    }

}
