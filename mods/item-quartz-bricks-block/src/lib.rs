use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct QuartzBricksBlockItem;

impl Item for QuartzBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:quartz_bricks_block",
        label: "Quartz Bricks",
    };
}

impl ItemRender for QuartzBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-quartz-bricks-block:item/quartz_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = QuartzBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <QuartzBricksBlockItem as ItemRender>::RENDER;

pub struct ItemQuartzBricksBlockMod;

impl ItemQuartzBricksBlockMod {
    pub fn init(_block: &mut block_quartz_bricks::BlockQuartzBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
