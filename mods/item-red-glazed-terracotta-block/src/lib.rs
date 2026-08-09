use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct RedGlazedTerracottaBlockItem;

impl Item for RedGlazedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:red_glazed_terracotta_block",
        label: "Red Glazed Terracotta",
    };
}

impl ItemRender for RedGlazedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-red-glazed-terracotta-block:item/red_glazed_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = RedGlazedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <RedGlazedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemRedGlazedTerracottaBlockMod;

impl ItemRedGlazedTerracottaBlockMod {
    pub fn init(_block: &mut block_red_glazed_terracotta::BlockRedGlazedTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
