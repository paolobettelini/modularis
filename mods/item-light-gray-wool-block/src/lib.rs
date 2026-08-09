use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LightGrayWoolBlockItem;

impl Item for LightGrayWoolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:light_gray_wool_block",
        label: "Light Gray Wool",
    };
}

impl ItemRender for LightGrayWoolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-light-gray-wool-block:item/light_gray_wool_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LightGrayWoolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <LightGrayWoolBlockItem as ItemRender>::RENDER;

pub struct ItemLightGrayWoolBlockMod;

impl ItemLightGrayWoolBlockMod {
    pub fn init(_block: &mut block_light_gray_wool::BlockLightGrayWoolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
