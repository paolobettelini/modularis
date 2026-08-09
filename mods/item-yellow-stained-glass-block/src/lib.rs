use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct YellowStainedGlassBlockItem;

impl Item for YellowStainedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:yellow_stained_glass_block",
        label: "Yellow Stained Glass",
    };
}

impl ItemRender for YellowStainedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-yellow-stained-glass-block:item/yellow_stained_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = YellowStainedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <YellowStainedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemYellowStainedGlassBlockMod;

impl ItemYellowStainedGlassBlockMod {
    pub fn init(_block: &mut block_yellow_stained_glass::BlockYellowStainedGlassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
