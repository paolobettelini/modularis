use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct SandBlockItem;
impl Item for SandBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:sand_block",
        label: "Sand",
    };
}
pub const ITEM_INFO: ItemInfo = SandBlockItem::INFO;
pub struct ItemSandBlockMod;
impl ItemSandBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
