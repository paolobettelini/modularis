use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ExposedCutCopperBlockItem;

impl Item for ExposedCutCopperBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:exposed_cut_copper_block",
        label: "Exposed Cut Copper",
    };
}

impl ItemRender for ExposedCutCopperBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-exposed-cut-copper-block:item/exposed_cut_copper_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ExposedCutCopperBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ExposedCutCopperBlockItem as ItemRender>::RENDER;

pub struct ItemExposedCutCopperBlockMod;

impl ItemExposedCutCopperBlockMod {
    pub fn init(_block: &mut block_exposed_cut_copper::BlockExposedCutCopperMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
