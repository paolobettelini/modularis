use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ExposedCopperGrateBlockItem;

impl Item for ExposedCopperGrateBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:exposed_copper_grate_block",
        label: "Exposed Copper Grate",
    };
}

impl ItemRender for ExposedCopperGrateBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-exposed-copper-grate-block:item/exposed_copper_grate_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ExposedCopperGrateBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ExposedCopperGrateBlockItem as ItemRender>::RENDER;

pub struct ItemExposedCopperGrateBlockMod;

impl ItemExposedCopperGrateBlockMod {
    pub fn init(_block: &mut block_exposed_copper_grate::BlockExposedCopperGrateMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
