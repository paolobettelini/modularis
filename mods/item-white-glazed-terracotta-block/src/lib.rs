use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct WhiteGlazedTerracottaBlockItem;

impl Item for WhiteGlazedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:white_glazed_terracotta_block",
        label: "White Glazed Terracotta",
    };
}

impl ItemRender for WhiteGlazedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-white-glazed-terracotta-block:item/white_glazed_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = WhiteGlazedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <WhiteGlazedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemWhiteGlazedTerracottaBlockMod;

impl ItemWhiteGlazedTerracottaBlockMod {
    pub fn init(_block: &mut block_white_glazed_terracotta::BlockWhiteGlazedTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
