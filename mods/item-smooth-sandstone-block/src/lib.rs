use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct SmoothSandstoneBlockItem;

impl Item for SmoothSandstoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:smooth_sandstone_block",
        label: "Smooth Sandstone",
    };
}

impl ItemRender for SmoothSandstoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-smooth-sandstone-block:item/smooth_sandstone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = SmoothSandstoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SmoothSandstoneBlockItem as ItemRender>::RENDER;

pub struct ItemSmoothSandstoneBlockMod;

impl ItemSmoothSandstoneBlockMod {
    pub fn init(_block: &mut block_smooth_sandstone::BlockSmoothSandstoneMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
