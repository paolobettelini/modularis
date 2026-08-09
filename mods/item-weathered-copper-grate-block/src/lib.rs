use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct WeatheredCopperGrateBlockItem;

impl Item for WeatheredCopperGrateBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:weathered_copper_grate_block",
        label: "Weathered Copper Grate",
    };
}

impl ItemRender for WeatheredCopperGrateBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-weathered-copper-grate-block:item/weathered_copper_grate_block"),
    };
}

pub const ITEM_INFO: ItemInfo = WeatheredCopperGrateBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <WeatheredCopperGrateBlockItem as ItemRender>::RENDER;

pub struct ItemWeatheredCopperGrateBlockMod;

impl ItemWeatheredCopperGrateBlockMod {
    pub fn init(_block: &mut block_weathered_copper_grate::BlockWeatheredCopperGrateMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
