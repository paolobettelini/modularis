use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct CrimsonNyliumBlockItem;
impl Item for CrimsonNyliumBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:crimson_nylium_block",
        label: "Crimson Nylium",
    };
}
pub const ITEM_INFO: ItemInfo = CrimsonNyliumBlockItem::INFO;
pub struct ItemCrimsonNyliumBlockMod;
impl ItemCrimsonNyliumBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
