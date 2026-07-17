use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct TerracottaBlockItem;
impl Item for TerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:terracotta_block",
        label: "Terracotta",
    };
}
pub const ITEM_INFO: ItemInfo = TerracottaBlockItem::INFO;
pub struct ItemTerracottaBlockMod;
impl ItemTerracottaBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
