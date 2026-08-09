use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LimeGlazedTerracottaBlockItem;

impl Item for LimeGlazedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:lime_glazed_terracotta_block",
        label: "Lime Glazed Terracotta",
    };
}

impl ItemRender for LimeGlazedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-lime-glazed-terracotta-block:item/lime_glazed_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LimeGlazedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <LimeGlazedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemLimeGlazedTerracottaBlockMod;

impl ItemLimeGlazedTerracottaBlockMod {
    pub fn init(_block: &mut block_lime_glazed_terracotta::BlockLimeGlazedTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
