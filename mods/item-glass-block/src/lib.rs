use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GlassBlockItem;

impl Item for GlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:glass_block",
        label: "Glass",
    };
}

impl ItemRender for GlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-glass-block:item/glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GlassBlockItem as ItemRender>::RENDER;

pub struct ItemGlassBlockMod;

impl ItemGlassBlockMod {
    pub fn init(_block: &mut block_glass::BlockGlassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
