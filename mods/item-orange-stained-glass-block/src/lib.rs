use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct OrangeStainedGlassBlockItem;

impl Item for OrangeStainedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:orange_stained_glass_block",
        label: "Orange Stained Glass",
    };
}

impl ItemRender for OrangeStainedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-orange-stained-glass-block:item/orange_stained_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = OrangeStainedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <OrangeStainedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemOrangeStainedGlassBlockMod;

impl ItemOrangeStainedGlassBlockMod {
    pub fn init(_block: &mut block_orange_stained_glass::BlockOrangeStainedGlassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
