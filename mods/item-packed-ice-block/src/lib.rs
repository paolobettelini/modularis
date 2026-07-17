use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct PackedIceBlockItem;
impl Item for PackedIceBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:packed_ice_block",
        label: "Packed Ice",
    };
}
pub const ITEM_INFO: ItemInfo = PackedIceBlockItem::INFO;
pub struct ItemPackedIceBlockMod;
impl ItemPackedIceBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
