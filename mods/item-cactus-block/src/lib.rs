use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct CactusBlockItem;
impl Item for CactusBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cactus_block",
        label: "Cactus",
    };
}
pub const ITEM_INFO: ItemInfo = CactusBlockItem::INFO;
pub struct ItemCactusBlockMod;
impl ItemCactusBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
