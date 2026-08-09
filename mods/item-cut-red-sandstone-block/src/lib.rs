use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CutRedSandstoneBlockItem;

impl Item for CutRedSandstoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cut_red_sandstone_block",
        label: "Cut Red Sandstone",
    };
}

impl ItemRender for CutRedSandstoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cut-red-sandstone-block:item/cut_red_sandstone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CutRedSandstoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CutRedSandstoneBlockItem as ItemRender>::RENDER;

pub struct ItemCutRedSandstoneBlockMod;

impl ItemCutRedSandstoneBlockMod {
    pub fn init(_block: &mut block_cut_red_sandstone::BlockCutRedSandstoneMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
