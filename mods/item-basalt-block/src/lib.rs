use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct BasaltBlockItem;
impl Item for BasaltBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:basalt_block",
        label: "Basalt",
    };
}
pub const ITEM_INFO: ItemInfo = BasaltBlockItem::INFO;
pub struct ItemBasaltBlockMod;
impl ItemBasaltBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
