use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct SmoothBasaltBlockItem;

impl Item for SmoothBasaltBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:smooth_basalt_block",
        label: "Smooth Basalt",
    };
}

impl ItemRender for SmoothBasaltBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-smooth-basalt-block:item/smooth_basalt_block"),
    };
}

pub const ITEM_INFO: ItemInfo = SmoothBasaltBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SmoothBasaltBlockItem as ItemRender>::RENDER;

pub struct ItemSmoothBasaltBlockMod;

impl ItemSmoothBasaltBlockMod {
    pub fn init(_block: &mut block_smooth_basalt::BlockSmoothBasaltMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
