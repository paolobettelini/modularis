use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct WhiteStainedGlassBlockItem;

impl Item for WhiteStainedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:white_stained_glass_block",
        label: "White Stained Glass",
    };
}

impl ItemRender for WhiteStainedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-white-stained-glass-block:item/white_stained_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = WhiteStainedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <WhiteStainedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemWhiteStainedGlassBlockMod;

impl ItemWhiteStainedGlassBlockMod {
    pub fn init(_block: &mut block_white_stained_glass::BlockWhiteStainedGlassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
