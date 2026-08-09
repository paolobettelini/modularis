use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct WeatheredCopperBlockItem;

impl Item for WeatheredCopperBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:weathered_copper_block",
        label: "Weathered Copper",
    };
}

impl ItemRender for WeatheredCopperBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-weathered-copper-block:item/weathered_copper_block"),
    };
}

pub const ITEM_INFO: ItemInfo = WeatheredCopperBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <WeatheredCopperBlockItem as ItemRender>::RENDER;

pub struct ItemWeatheredCopperBlockMod;

impl ItemWeatheredCopperBlockMod {
    pub fn init(_block: &mut block_weathered_copper::BlockWeatheredCopperMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
