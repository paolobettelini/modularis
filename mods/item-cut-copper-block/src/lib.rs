use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CutCopperBlockItem;

impl Item for CutCopperBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cut_copper_block",
        label: "Cut Copper",
    };
}

impl ItemRender for CutCopperBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cut-copper-block:item/cut_copper_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CutCopperBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CutCopperBlockItem as ItemRender>::RENDER;

pub struct ItemCutCopperBlockMod;

impl ItemCutCopperBlockMod {
    pub fn init(_block: &mut block_cut_copper::BlockCutCopperMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
