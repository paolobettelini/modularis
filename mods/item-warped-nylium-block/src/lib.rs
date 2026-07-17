use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct WarpedNyliumBlockItem;
impl Item for WarpedNyliumBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:warped_nylium_block",
        label: "Warped Nylium",
    };
}
pub const ITEM_INFO: ItemInfo = WarpedNyliumBlockItem::INFO;
pub struct ItemWarpedNyliumBlockMod;
impl ItemWarpedNyliumBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
