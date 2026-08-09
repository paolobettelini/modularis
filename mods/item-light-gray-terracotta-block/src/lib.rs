use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LightGrayTerracottaBlockItem;

impl Item for LightGrayTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:light_gray_terracotta_block",
        label: "Light Gray Terracotta",
    };
}

impl ItemRender for LightGrayTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-light-gray-terracotta-block:item/light_gray_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LightGrayTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <LightGrayTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemLightGrayTerracottaBlockMod;

impl ItemLightGrayTerracottaBlockMod {
    pub fn init(_block: &mut block_light_gray_terracotta::BlockLightGrayTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
