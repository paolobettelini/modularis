use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PurpleStainedGlassBlockItem;

impl Item for PurpleStainedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:purple_stained_glass_block",
        label: "Purple Stained Glass",
    };
}

impl ItemRender for PurpleStainedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-purple-stained-glass-block:item/purple_stained_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PurpleStainedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PurpleStainedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemPurpleStainedGlassBlockMod;

impl ItemPurpleStainedGlassBlockMod {
    pub fn init(_block: &mut block_purple_stained_glass::BlockPurpleStainedGlassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
