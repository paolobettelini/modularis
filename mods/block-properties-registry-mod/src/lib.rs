use bevy_mod::BevyMod;
use block_properties_api::BlockProperties;
use tokio::task::JoinHandle;

pub struct BlockPropertiesRegistryMod;

impl BlockPropertiesRegistryMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.init_resource::<BlockProperties>();
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}
