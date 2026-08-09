use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DeadTubeCoralBlockItem;

impl Item for DeadTubeCoralBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:dead_tube_coral_block",
        label: "Dead Tube Coral Block",
    };
}

impl ItemRender for DeadTubeCoralBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-dead-tube-coral-block:item/dead_tube_coral_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DeadTubeCoralBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DeadTubeCoralBlockItem as ItemRender>::RENDER;

pub struct ItemDeadTubeCoralBlockMod;

impl ItemDeadTubeCoralBlockMod {
    pub fn init(_block: &mut block_dead_tube_coral_block::BlockDeadTubeCoralBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
