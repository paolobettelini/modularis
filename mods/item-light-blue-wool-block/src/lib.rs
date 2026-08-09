use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LightBlueWoolBlockItem;

impl Item for LightBlueWoolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:light_blue_wool_block",
        label: "Light Blue Wool",
    };
}

impl ItemRender for LightBlueWoolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-light-blue-wool-block:item/light_blue_wool_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LightBlueWoolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <LightBlueWoolBlockItem as ItemRender>::RENDER;

pub struct ItemLightBlueWoolBlockMod;

impl ItemLightBlueWoolBlockMod {
    pub fn init(_block: &mut block_light_blue_wool::BlockLightBlueWoolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
