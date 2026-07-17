use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct BirchLeavesBlockItem;
impl Item for BirchLeavesBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:birch_leaves_block",
        label: "Birch Leaves",
    };
}
pub const ITEM_INFO: ItemInfo = BirchLeavesBlockItem::INFO;
pub struct ItemBirchLeavesBlockMod;
impl ItemBirchLeavesBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
