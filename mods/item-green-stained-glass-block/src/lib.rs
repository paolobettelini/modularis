use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GreenStainedGlassBlockItem;

impl Item for GreenStainedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:green_stained_glass_block",
        label: "Green Stained Glass",
    };
}

impl ItemRender for GreenStainedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-green-stained-glass-block:item/green_stained_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GreenStainedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GreenStainedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemGreenStainedGlassBlockMod;

impl ItemGreenStainedGlassBlockMod {
    pub fn init(_block: &mut block_green_stained_glass::BlockGreenStainedGlassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
