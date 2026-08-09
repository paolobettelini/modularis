use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct StrippedWarpedStemBlockItem;

impl Item for StrippedWarpedStemBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:stripped_warped_stem_block",
        label: "Stripped Warped Stem",
    };
}

impl ItemRender for StrippedWarpedStemBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-stripped-warped-stem-block:item/stripped_warped_stem_block"),
    };
}

pub const ITEM_INFO: ItemInfo = StrippedWarpedStemBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <StrippedWarpedStemBlockItem as ItemRender>::RENDER;

pub struct ItemStrippedWarpedStemBlockMod;

impl ItemStrippedWarpedStemBlockMod {
    pub fn init(_block: &mut block_stripped_warped_stem::BlockStrippedWarpedStemMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
