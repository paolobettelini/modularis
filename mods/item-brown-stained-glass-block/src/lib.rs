use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BrownStainedGlassBlockItem;

impl Item for BrownStainedGlassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:brown_stained_glass_block",
        label: "Brown Stained Glass",
    };
}

impl ItemRender for BrownStainedGlassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-brown-stained-glass-block:item/brown_stained_glass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BrownStainedGlassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BrownStainedGlassBlockItem as ItemRender>::RENDER;

pub struct ItemBrownStainedGlassBlockMod;

impl ItemBrownStainedGlassBlockMod {
    pub fn init(_block: &mut block_brown_stained_glass::BlockBrownStainedGlassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
