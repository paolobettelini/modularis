use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct SnowBlockItem;
impl Item for SnowBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:snow_block",
        label: "Snow",
    };
}
pub const ITEM_INFO: ItemInfo = SnowBlockItem::INFO;
pub struct ItemSnowBlockMod;
impl ItemSnowBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
