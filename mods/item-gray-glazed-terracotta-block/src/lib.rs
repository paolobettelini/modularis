use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GrayGlazedTerracottaBlockItem;

impl Item for GrayGlazedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:gray_glazed_terracotta_block",
        label: "Gray Glazed Terracotta",
    };
}

impl ItemRender for GrayGlazedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-gray-glazed-terracotta-block:item/gray_glazed_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GrayGlazedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GrayGlazedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemGrayGlazedTerracottaBlockMod;

impl ItemGrayGlazedTerracottaBlockMod {
    pub fn init(_block: &mut block_gray_glazed_terracotta::BlockGrayGlazedTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
