use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;

pub struct NetherrackBlockItem;

impl Item for NetherrackBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:netherrack_block",
        label: "Netherrack",
    };
}

pub const ITEM_INFO: ItemInfo = NetherrackBlockItem::INFO;

pub struct ItemNetherrackBlockMod;

impl ItemNetherrackBlockMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
