use bevy_mod::BevyMod;
use block_component_api::BlockComponentRegistry;
use tokio::task::JoinHandle;

pub struct BlockComponentRegistryMod;
impl BlockComponentRegistryMod {
    pub fn init(bevy: &mut BevyMod) -> Self { bevy.app.init_resource::<BlockComponentRegistry>(); Self }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

