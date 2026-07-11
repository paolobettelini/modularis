use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;

pub struct DirtBlockItem;

impl Item for DirtBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:dirt_block",
        label: "Dirt",
    };
}

pub const ITEM_INFO: ItemInfo = DirtBlockItem::INFO;

pub struct ItemDirtBlockMod;

impl ItemDirtBlockMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
