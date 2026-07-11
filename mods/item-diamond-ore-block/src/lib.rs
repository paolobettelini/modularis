use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;

pub struct DiamondOreBlockItem;

impl Item for DiamondOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:diamond_ore_block",
        label: "Diamond Ore",
    };
}

pub const ITEM_INFO: ItemInfo = DiamondOreBlockItem::INFO;

pub struct ItemDiamondOreBlockMod;

impl ItemDiamondOreBlockMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
