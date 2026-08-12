use bevy_mod::BevyMod;
use block_durability_api::{BlockDurability, BlockDurabilityProperty};
use block_properties_api::BlockProperties;
use block_properties_registry_mod::BlockPropertiesRegistryMod;
use generated_block_registry::BlockId;
use tokio::task::JoinHandle;

pub struct BlockDurabilityShortGrassVanillaMod;

impl BlockDurabilityShortGrassVanillaMod {
    pub fn init(
        bevy: &mut BevyMod,
        _properties: &mut BlockPropertiesRegistryMod,
        _short_grass: &mut block_short_grass::BlockShortGrassMod,
    ) -> Self {
        bevy.app
            .world_mut()
            .resource_mut::<BlockProperties>()
            .set::<BlockDurabilityProperty>(BlockId::ShortGrass, BlockDurability::breakable(0));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}
