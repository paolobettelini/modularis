use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct SmoothRedSandstoneBlockItem;

impl Item for SmoothRedSandstoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:smooth_red_sandstone_block",
        label: "Smooth Red Sandstone",
    };
}

impl ItemRender for SmoothRedSandstoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-smooth-red-sandstone-block:item/smooth_red_sandstone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = SmoothRedSandstoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SmoothRedSandstoneBlockItem as ItemRender>::RENDER;

pub struct ItemSmoothRedSandstoneBlockMod;

impl ItemSmoothRedSandstoneBlockMod {
    pub fn init(_block: &mut block_smooth_red_sandstone::BlockSmoothRedSandstoneMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
