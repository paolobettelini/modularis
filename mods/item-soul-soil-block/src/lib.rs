use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct SoulSoilBlockItem;
impl Item for SoulSoilBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:soul_soil_block",
        label: "Soul Soil",
    };
}
pub const ITEM_INFO: ItemInfo = SoulSoilBlockItem::INFO;
pub struct ItemSoulSoilBlockMod;
impl ItemSoulSoilBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
