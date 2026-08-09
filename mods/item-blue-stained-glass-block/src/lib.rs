use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BlueStainedGlassBlockItem;

impl Item for BlueStainedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:blue_stained_glass_block",
        label: "Blue Stained Glass",
    };
}

impl ItemRender for BlueStainedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-blue-stained-glass-block:item/blue_stained_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BlueStainedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BlueStainedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemBlueStainedGlassBlockMod;

impl ItemBlueStainedGlassBlockMod {
    pub fn init(_block: &mut block_blue_stained_glass::BlockBlueStainedGlassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
