use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DeadBubbleCoralBlockItem;

impl Item for DeadBubbleCoralBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:dead_bubble_coral_block",
        label: "Dead Bubble Coral Block",
    };
}

impl ItemRender for DeadBubbleCoralBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-dead-bubble-coral-block:item/dead_bubble_coral_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DeadBubbleCoralBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DeadBubbleCoralBlockItem as ItemRender>::RENDER;

pub struct ItemDeadBubbleCoralBlockMod;

impl ItemDeadBubbleCoralBlockMod {
    pub fn init(_block: &mut block_dead_bubble_coral_block::BlockDeadBubbleCoralBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
