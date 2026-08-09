use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DeadBrainCoralBlockItem;

impl Item for DeadBrainCoralBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:dead_brain_coral_block",
        label: "Dead Brain Coral Block",
    };
}

impl ItemRender for DeadBrainCoralBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-dead-brain-coral-block:item/dead_brain_coral_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DeadBrainCoralBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DeadBrainCoralBlockItem as ItemRender>::RENDER;

pub struct ItemDeadBrainCoralBlockMod;

impl ItemDeadBrainCoralBlockMod {
    pub fn init(_block: &mut block_dead_brain_coral_block::BlockDeadBrainCoralBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
