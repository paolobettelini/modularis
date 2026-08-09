use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct WeatheredCutCopperBlockItem;

impl Item for WeatheredCutCopperBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:weathered_cut_copper_block",
        label: "Weathered Cut Copper",
    };
}

impl ItemRender for WeatheredCutCopperBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-weathered-cut-copper-block:item/weathered_cut_copper_block"),
    };
}

pub const ITEM_INFO: ItemInfo = WeatheredCutCopperBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <WeatheredCutCopperBlockItem as ItemRender>::RENDER;

pub struct ItemWeatheredCutCopperBlockMod;

impl ItemWeatheredCutCopperBlockMod {
    pub fn init(_block: &mut block_weathered_cut_copper::BlockWeatheredCutCopperMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
