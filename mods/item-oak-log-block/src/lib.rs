use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct OakLogBlockItem;
impl Item for OakLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:oak_log_block",
        label: "Oak Log",
    };
}
pub const ITEM_INFO: ItemInfo = OakLogBlockItem::INFO;
pub struct ItemOakLogBlockMod;
impl ItemOakLogBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
