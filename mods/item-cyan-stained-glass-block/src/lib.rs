use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CyanStainedGlassBlockItem;

impl Item for CyanStainedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cyan_stained_glass_block",
        label: "Cyan Stained Glass",
    };
}

impl ItemRender for CyanStainedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cyan-stained-glass-block:item/cyan_stained_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CyanStainedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CyanStainedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemCyanStainedGlassBlockMod;

impl ItemCyanStainedGlassBlockMod {
    pub fn init(_block: &mut block_cyan_stained_glass::BlockCyanStainedGlassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
