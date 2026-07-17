use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct BirchLogBlockItem;
impl Item for BirchLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:birch_log_block",
        label: "Birch Log",
    };
}
pub const ITEM_INFO: ItemInfo = BirchLogBlockItem::INFO;
pub struct ItemBirchLogBlockMod;
impl ItemBirchLogBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
