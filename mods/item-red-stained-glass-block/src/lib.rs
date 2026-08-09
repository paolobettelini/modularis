use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct RedStainedGlassBlockItem;

impl Item for RedStainedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:red_stained_glass_block",
        label: "Red Stained Glass",
    };
}

impl ItemRender for RedStainedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-red-stained-glass-block:item/red_stained_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = RedStainedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <RedStainedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemRedStainedGlassBlockMod;

impl ItemRedStainedGlassBlockMod {
    pub fn init(_block: &mut block_red_stained_glass::BlockRedStainedGlassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
