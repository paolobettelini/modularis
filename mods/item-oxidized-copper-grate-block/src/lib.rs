use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct OxidizedCopperGrateBlockItem;

impl Item for OxidizedCopperGrateBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:oxidized_copper_grate_block",
        label: "Oxidized Copper Grate",
    };
}

impl ItemRender for OxidizedCopperGrateBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-oxidized-copper-grate-block:item/oxidized_copper_grate_block"),
    };
}

pub const ITEM_INFO: ItemInfo = OxidizedCopperGrateBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <OxidizedCopperGrateBlockItem as ItemRender>::RENDER;

pub struct ItemOxidizedCopperGrateBlockMod;

impl ItemOxidizedCopperGrateBlockMod {
    pub fn init(_block: &mut block_oxidized_copper_grate::BlockOxidizedCopperGrateMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
