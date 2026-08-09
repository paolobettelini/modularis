use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DeepslateTilesBlockItem;

impl Item for DeepslateTilesBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:deepslate_tiles_block",
        label: "Deepslate Tiles",
    };
}

impl ItemRender for DeepslateTilesBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-deepslate-tiles-block:item/deepslate_tiles_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DeepslateTilesBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DeepslateTilesBlockItem as ItemRender>::RENDER;

pub struct ItemDeepslateTilesBlockMod;

impl ItemDeepslateTilesBlockMod {
    pub fn init(_block: &mut block_deepslate_tiles::BlockDeepslateTilesMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
