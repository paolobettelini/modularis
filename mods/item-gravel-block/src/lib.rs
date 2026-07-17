use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct GravelBlockItem;
impl Item for GravelBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:gravel_block",
        label: "Gravel",
    };
}
pub const ITEM_INFO: ItemInfo = GravelBlockItem::INFO;
pub struct ItemGravelBlockMod;
impl ItemGravelBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
