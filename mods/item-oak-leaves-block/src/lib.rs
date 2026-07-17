use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct OakLeavesBlockItem;
impl Item for OakLeavesBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:oak_leaves_block",
        label: "Oak Leaves",
    };
}
pub const ITEM_INFO: ItemInfo = OakLeavesBlockItem::INFO;
pub struct ItemOakLeavesBlockMod;
impl ItemOakLeavesBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
