use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LightBlueTerracottaBlockItem;

impl Item for LightBlueTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:light_blue_terracotta_block",
        label: "Light Blue Terracotta",
    };
}

impl ItemRender for LightBlueTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-light-blue-terracotta-block:item/light_blue_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LightBlueTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <LightBlueTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemLightBlueTerracottaBlockMod;

impl ItemLightBlueTerracottaBlockMod {
    pub fn init(_block: &mut block_light_blue_terracotta::BlockLightBlueTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
