use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CrackedDeepslateTilesBlockItem;

impl Item for CrackedDeepslateTilesBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cracked_deepslate_tiles_block",
        label: "Cracked Deepslate Tiles",
    };
}

impl ItemRender for CrackedDeepslateTilesBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cracked-deepslate-tiles-block:item/cracked_deepslate_tiles_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CrackedDeepslateTilesBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CrackedDeepslateTilesBlockItem as ItemRender>::RENDER;

pub struct ItemCrackedDeepslateTilesBlockMod;

impl ItemCrackedDeepslateTilesBlockMod {
    pub fn init(_block: &mut block_cracked_deepslate_tiles::BlockCrackedDeepslateTilesMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
