use bevy_mod::BevyMod;
use block_durability_api::{BlockDurability, BlockDurabilityProperty};
use block_properties_api::BlockProperties;
use block_properties_registry_mod::BlockPropertiesRegistryMod;
use generated_block_registry::BlockId;
use tokio::task::JoinHandle;

pub struct BlockDurabilityCobblestoneVanillaMod;

impl BlockDurabilityCobblestoneVanillaMod {
    pub fn init(
        bevy: &mut BevyMod,
        _properties: &mut BlockPropertiesRegistryMod,
        _cobblestone: &mut block_cobblestone::BlockCobblestoneMod,
    ) -> Self {
        bevy.app
            .world_mut()
            .resource_mut::<BlockProperties>()
            .set::<BlockDurabilityProperty>(BlockId::Cobblestone, BlockDurability::breakable(160));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
