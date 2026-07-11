use item_api::ItemInfo;
use item_manager_api::{ItemId, ItemManagerApi};
use item_registry_codegen::ItemRegistryCodegenMod;
use tokio::task::JoinHandle;

pub struct GeneratedItemManager;

impl GeneratedItemManager {
    pub fn init(_codegen: &mut ItemRegistryCodegenMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ItemManagerApi for GeneratedItemManager {
    fn info(item: ItemId) -> &'static ItemInfo {
        generated_item_registry::info(item)
    }

    fn all() -> &'static [ItemId] {
        generated_item_registry::all_items()
    }

    fn from_string(id: &str) -> Option<ItemId> {
        generated_item_registry::from_str(id)
    }

    fn id(item: ItemId) -> &'static str {
        generated_item_registry::id(item)
    }

    fn label(item: ItemId) -> &'static str {
        generated_item_registry::label(item)
    }
}
