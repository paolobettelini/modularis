use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;

pub struct DiamondBlockItem;

impl Item for DiamondBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:diamond_block",
        label: "Diamond Block",
    };
}

pub const ITEM_INFO: ItemInfo = DiamondBlockItem::INFO;

pub struct ItemDiamondBlockMod;

impl ItemDiamondBlockMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
