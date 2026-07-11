use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_registry_codegen::BlockRegistryCodegenMod;
use generated_network_messages::NetworkMessageEventsPlugin;
use item_metadata_registry_codegen::ItemMetadataRegistryCodegenMod;
use item_registry_codegen::ItemRegistryCodegenMod;
use tokio::task::JoinHandle;

pub struct NetworkProtocolMod;

impl NetworkProtocolMod {
    pub fn init(
        bevy: &mut BevyMod,
        _blocks: &mut BlockRegistryCodegenMod,
        _items: &mut ItemRegistryCodegenMod,
        _metadata: &mut ItemMetadataRegistryCodegenMod,
    ) -> Self {
        bevy.app.add_plugins(NetworkMessageEventsPlugin);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
