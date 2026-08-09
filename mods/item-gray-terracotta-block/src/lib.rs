use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GrayTerracottaBlockItem;

impl Item for GrayTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:gray_terracotta_block",
        label: "Gray Terracotta",
    };
}

impl ItemRender for GrayTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-gray-terracotta-block:item/gray_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GrayTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GrayTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemGrayTerracottaBlockMod;

impl ItemGrayTerracottaBlockMod {
    pub fn init(_block: &mut block_gray_terracotta::BlockGrayTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
