use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;

pub struct GrassBlockItem;

impl Item for GrassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:grass_block",
        label: "Grass",
    };
}

pub const ITEM_INFO: ItemInfo = GrassBlockItem::INFO;

pub struct ItemGrassBlockMod;

impl ItemGrassBlockMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
