use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LimeStainedGlassBlockItem;

impl Item for LimeStainedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:lime_stained_glass_block",
        label: "Lime Stained Glass",
    };
}

impl ItemRender for LimeStainedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-lime-stained-glass-block:item/lime_stained_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LimeStainedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <LimeStainedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemLimeStainedGlassBlockMod;

impl ItemLimeStainedGlassBlockMod {
    pub fn init(_block: &mut block_lime_stained_glass::BlockLimeStainedGlassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
