use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CyanGlazedTerracottaBlockItem;

impl Item for CyanGlazedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cyan_glazed_terracotta_block",
        label: "Cyan Glazed Terracotta",
    };
}

impl ItemRender for CyanGlazedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cyan-glazed-terracotta-block:item/cyan_glazed_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CyanGlazedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CyanGlazedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemCyanGlazedTerracottaBlockMod;

impl ItemCyanGlazedTerracottaBlockMod {
    pub fn init(_block: &mut block_cyan_glazed_terracotta::BlockCyanGlazedTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
