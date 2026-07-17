use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct MossBlockItem;
impl Item for MossBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:moss_block",
        label: "Moss",
    };
}
pub const ITEM_INFO: ItemInfo = MossBlockItem::INFO;
pub struct ItemMossBlockMod;
impl ItemMossBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
