use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct RedSandBlockItem;
impl Item for RedSandBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:red_sand_block",
        label: "Red Sand",
    };
}
pub const ITEM_INFO: ItemInfo = RedSandBlockItem::INFO;
pub struct ItemRedSandBlockMod;
impl ItemRedSandBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
