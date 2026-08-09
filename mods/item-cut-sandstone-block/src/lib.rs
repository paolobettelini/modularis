use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CutSandstoneBlockItem;

impl Item for CutSandstoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cut_sandstone_block",
        label: "Cut Sandstone",
    };
}

impl ItemRender for CutSandstoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cut-sandstone-block:item/cut_sandstone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CutSandstoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CutSandstoneBlockItem as ItemRender>::RENDER;

pub struct ItemCutSandstoneBlockMod;

impl ItemCutSandstoneBlockMod {
    pub fn init(_block: &mut block_cut_sandstone::BlockCutSandstoneMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
