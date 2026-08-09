use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CopperGrateBlockItem;

impl Item for CopperGrateBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:copper_grate_block",
        label: "Copper Grate",
    };
}

impl ItemRender for CopperGrateBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-copper-grate-block:item/copper_grate_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CopperGrateBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CopperGrateBlockItem as ItemRender>::RENDER;

pub struct ItemCopperGrateBlockMod;

impl ItemCopperGrateBlockMod {
    pub fn init(_block: &mut block_copper_grate::BlockCopperGrateMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
