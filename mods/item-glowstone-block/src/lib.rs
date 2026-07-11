use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;

pub struct GlowstoneBlockItem;

impl Item for GlowstoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:glowstone_block",
        label: "Glowstone",
    };
}

pub const ITEM_INFO: ItemInfo = GlowstoneBlockItem::INFO;

pub struct ItemGlowstoneBlockMod;

impl ItemGlowstoneBlockMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
