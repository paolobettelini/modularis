use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;

pub struct EndStoneBlockItem;

impl Item for EndStoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:end_stone_block",
        label: "End Stone",
    };
}

pub const ITEM_INFO: ItemInfo = EndStoneBlockItem::INFO;

pub struct ItemEndStoneBlockMod;

impl ItemEndStoneBlockMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
