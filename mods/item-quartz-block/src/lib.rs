use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct QuartzBlockItem;

impl Item for QuartzBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:quartz_block",
        label: "Quartz Block",
    };
}

impl ItemRender for QuartzBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-quartz-block:item/quartz_block"),
    };
}

pub const ITEM_INFO: ItemInfo = QuartzBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <QuartzBlockItem as ItemRender>::RENDER;

pub struct ItemQuartzBlockMod;

impl ItemQuartzBlockMod {
    pub fn init(_block: &mut block_quartz_block::BlockQuartzBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
