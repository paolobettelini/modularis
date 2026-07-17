use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct SoulSandBlockItem;
impl Item for SoulSandBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:soul_sand_block",
        label: "Soul Sand",
    };
}
pub const ITEM_INFO: ItemInfo = SoulSandBlockItem::INFO;
pub struct ItemSoulSandBlockMod;
impl ItemSoulSandBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
