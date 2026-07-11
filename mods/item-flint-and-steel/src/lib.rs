use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;

pub struct FlintAndSteelItem;

impl Item for FlintAndSteelItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:flint-and-steel",
        label: "Flint and Steel",
    };
}

pub const ITEM_INFO: ItemInfo = FlintAndSteelItem::INFO;

pub struct ItemFlintAndSteelMod;

impl ItemFlintAndSteelMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
