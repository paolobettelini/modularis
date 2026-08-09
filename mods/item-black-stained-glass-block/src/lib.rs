use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BlackStainedGlassBlockItem;

impl Item for BlackStainedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:black_stained_glass_block",
        label: "Black Stained Glass",
    };
}

impl ItemRender for BlackStainedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-black-stained-glass-block:item/black_stained_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BlackStainedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BlackStainedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemBlackStainedGlassBlockMod;

impl ItemBlackStainedGlassBlockMod {
    pub fn init(_block: &mut block_black_stained_glass::BlockBlackStainedGlassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
