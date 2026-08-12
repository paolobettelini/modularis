use bevy_mod::BevyMod;
use block_durability_api::{BlockDurability, BlockDurabilityProperty};
use block_properties_api::BlockProperties;
use block_properties_registry_mod::BlockPropertiesRegistryMod;
use generated_block_registry::BlockId;
use tokio::task::JoinHandle;

pub struct BlockDurabilityGrassVanillaMod;

impl BlockDurabilityGrassVanillaMod {
    pub fn init(
        bevy: &mut BevyMod,
        _properties: &mut BlockPropertiesRegistryMod,
        _grass: &mut block_grass::BlockGrassMod,
    ) -> Self {
        bevy.app
            .world_mut()
            .resource_mut::<BlockProperties>()
            .set::<BlockDurabilityProperty>(BlockId::Grass, BlockDurability::breakable(45));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
