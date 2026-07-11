use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;

pub struct ObsidianBlockItem;

impl Item for ObsidianBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:obsidian_block",
        label: "Obsidian",
    };
}

pub const ITEM_INFO: ItemInfo = ObsidianBlockItem::INFO;

pub struct ItemObsidianBlockMod;

impl ItemObsidianBlockMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
