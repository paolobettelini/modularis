use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LimeTerracottaBlockItem;

impl Item for LimeTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:lime_terracotta_block",
        label: "Lime Terracotta",
    };
}

impl ItemRender for LimeTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-lime-terracotta-block:item/lime_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LimeTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <LimeTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemLimeTerracottaBlockMod;

impl ItemLimeTerracottaBlockMod {
    pub fn init(_block: &mut block_lime_terracotta::BlockLimeTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
