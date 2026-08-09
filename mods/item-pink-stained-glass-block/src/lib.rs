use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PinkStainedGlassBlockItem;

impl Item for PinkStainedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:pink_stained_glass_block",
        label: "Pink Stained Glass",
    };
}

impl ItemRender for PinkStainedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-pink-stained-glass-block:item/pink_stained_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PinkStainedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PinkStainedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemPinkStainedGlassBlockMod;

impl ItemPinkStainedGlassBlockMod {
    pub fn init(_block: &mut block_pink_stained_glass::BlockPinkStainedGlassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
