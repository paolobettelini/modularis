use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct WarpedWartBlockItem;

impl Item for WarpedWartBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:warped_wart_block",
        label: "Warped Wart Block",
    };
}

impl ItemRender for WarpedWartBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-warped-wart-block:item/warped_wart_block"),
    };
}

pub const ITEM_INFO: ItemInfo = WarpedWartBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <WarpedWartBlockItem as ItemRender>::RENDER;

pub struct ItemWarpedWartBlockMod;

impl ItemWarpedWartBlockMod {
    pub fn init(_block: &mut block_warped_wart_block::BlockWarpedWartBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
