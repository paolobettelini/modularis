use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BubbleCoralBlockItem;

impl Item for BubbleCoralBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:bubble_coral_block",
        label: "Bubble Coral Block",
    };
}

impl ItemRender for BubbleCoralBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-bubble-coral-block:item/bubble_coral_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BubbleCoralBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BubbleCoralBlockItem as ItemRender>::RENDER;

pub struct ItemBubbleCoralBlockMod;

impl ItemBubbleCoralBlockMod {
    pub fn init(_block: &mut block_bubble_coral_block::BlockBubbleCoralBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
