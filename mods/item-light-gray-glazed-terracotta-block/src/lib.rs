use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LightGrayGlazedTerracottaBlockItem;

impl Item for LightGrayGlazedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:light_gray_glazed_terracotta_block",
        label: "Light Gray Glazed Terracotta",
    };
}

impl ItemRender for LightGrayGlazedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some(
            "item-light-gray-glazed-terracotta-block:item/light_gray_glazed_terracotta_block",
        ),
    };
}

pub const ITEM_INFO: ItemInfo = LightGrayGlazedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <LightGrayGlazedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemLightGrayGlazedTerracottaBlockMod;

impl ItemLightGrayGlazedTerracottaBlockMod {
    pub fn init(
        _block: &mut block_light_gray_glazed_terracotta::BlockLightGrayGlazedTerracottaMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
