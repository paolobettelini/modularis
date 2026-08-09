use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BlackGlazedTerracottaBlockItem;

impl Item for BlackGlazedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:black_glazed_terracotta_block",
        label: "Black Glazed Terracotta",
    };
}

impl ItemRender for BlackGlazedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-black-glazed-terracotta-block:item/black_glazed_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BlackGlazedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BlackGlazedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemBlackGlazedTerracottaBlockMod;

impl ItemBlackGlazedTerracottaBlockMod {
    pub fn init(_block: &mut block_black_glazed_terracotta::BlockBlackGlazedTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
