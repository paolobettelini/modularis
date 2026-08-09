use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct SmoothQuartzBlockItem;

impl Item for SmoothQuartzBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:smooth_quartz_block",
        label: "Smooth Quartz",
    };
}

impl ItemRender for SmoothQuartzBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-smooth-quartz-block:item/smooth_quartz_block"),
    };
}

pub const ITEM_INFO: ItemInfo = SmoothQuartzBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SmoothQuartzBlockItem as ItemRender>::RENDER;

pub struct ItemSmoothQuartzBlockMod;

impl ItemSmoothQuartzBlockMod {
    pub fn init(_block: &mut block_smooth_quartz::BlockSmoothQuartzMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
