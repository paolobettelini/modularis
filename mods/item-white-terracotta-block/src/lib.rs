use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct WhiteTerracottaBlockItem;

impl Item for WhiteTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:white_terracotta_block",
        label: "White Terracotta",
    };
}

impl ItemRender for WhiteTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-white-terracotta-block:item/white_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = WhiteTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <WhiteTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemWhiteTerracottaBlockMod;

impl ItemWhiteTerracottaBlockMod {
    pub fn init(_block: &mut block_white_terracotta::BlockWhiteTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
