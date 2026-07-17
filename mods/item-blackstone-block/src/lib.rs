use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct BlackstoneBlockItem;
impl Item for BlackstoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:blackstone_block",
        label: "Blackstone",
    };
}
pub const ITEM_INFO: ItemInfo = BlackstoneBlockItem::INFO;
pub struct ItemBlackstoneBlockMod;
impl ItemBlackstoneBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
