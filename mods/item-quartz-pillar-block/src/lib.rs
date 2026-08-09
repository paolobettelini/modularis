use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct QuartzPillarBlockItem;

impl Item for QuartzPillarBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:quartz_pillar_block",
        label: "Quartz Pillar",
    };
}

impl ItemRender for QuartzPillarBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-quartz-pillar-block:item/quartz_pillar_block"),
    };
}

pub const ITEM_INFO: ItemInfo = QuartzPillarBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <QuartzPillarBlockItem as ItemRender>::RENDER;

pub struct ItemQuartzPillarBlockMod;

impl ItemQuartzPillarBlockMod {
    pub fn init(_block: &mut block_quartz_pillar::BlockQuartzPillarMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
