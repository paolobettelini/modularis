use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;

pub struct StoneBlockItem;

impl Item for StoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:stone_block",
        label: "Stone",
    };
}

pub const ITEM_INFO: ItemInfo = StoneBlockItem::INFO;

pub struct ItemStoneBlockMod;

impl ItemStoneBlockMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
