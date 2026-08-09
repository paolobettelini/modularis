use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LightBlueGlazedTerracottaBlockItem;

impl Item for LightBlueGlazedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:light_blue_glazed_terracotta_block",
        label: "Light Blue Glazed Terracotta",
    };
}

impl ItemRender for LightBlueGlazedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some(
            "item-light-blue-glazed-terracotta-block:item/light_blue_glazed_terracotta_block",
        ),
    };
}

pub const ITEM_INFO: ItemInfo = LightBlueGlazedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <LightBlueGlazedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemLightBlueGlazedTerracottaBlockMod;

impl ItemLightBlueGlazedTerracottaBlockMod {
    pub fn init(
        _block: &mut block_light_blue_glazed_terracotta::BlockLightBlueGlazedTerracottaMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
