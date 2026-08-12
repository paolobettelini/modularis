use bevy_mod::BevyMod;
use block_component_api::BlockComponentRegistry;
use block_component_registry_mod::BlockComponentRegistryMod;
use block_durability_api::BlockDamage;
use tokio::task::JoinHandle;

pub struct BlockDamageComponentMod;
impl BlockDamageComponentMod {
    pub fn init(bevy: &mut BevyMod, _registry: &mut BlockComponentRegistryMod) -> Self {
        bevy.app.world_mut().resource_mut::<BlockComponentRegistry>().register::<BlockDamage>();
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

