use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BrainCoralBlockItem;

impl Item for BrainCoralBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:brain_coral_block",
        label: "Brain Coral Block",
    };
}

impl ItemRender for BrainCoralBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-brain-coral-block:item/brain_coral_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BrainCoralBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BrainCoralBlockItem as ItemRender>::RENDER;

pub struct ItemBrainCoralBlockMod;

impl ItemBrainCoralBlockMod {
    pub fn init(_block: &mut block_brain_coral_block::BlockBrainCoralBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
