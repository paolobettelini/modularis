use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;

pub struct CraftingTableBlockItem;

impl Item for CraftingTableBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:crafting_table_block",
        label: "Crafting Table",
    };
}

pub const ITEM_INFO: ItemInfo = CraftingTableBlockItem::INFO;

pub struct ItemCraftingTableBlockMod;

impl ItemCraftingTableBlockMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
