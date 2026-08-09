use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct WarpedStemBlockItem;

impl Item for WarpedStemBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:warped_stem_block",
        label: "Warped Stem",
    };
}

impl ItemRender for WarpedStemBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-warped-stem-block:item/warped_stem_block"),
    };
}

pub const ITEM_INFO: ItemInfo = WarpedStemBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <WarpedStemBlockItem as ItemRender>::RENDER;

pub struct ItemWarpedStemBlockMod;

impl ItemWarpedStemBlockMod {
    pub fn init(_block: &mut block_warped_stem::BlockWarpedStemMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
