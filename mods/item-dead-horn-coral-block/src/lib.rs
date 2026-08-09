use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DeadHornCoralBlockItem;

impl Item for DeadHornCoralBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:dead_horn_coral_block",
        label: "Dead Horn Coral Block",
    };
}

impl ItemRender for DeadHornCoralBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-dead-horn-coral-block:item/dead_horn_coral_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DeadHornCoralBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DeadHornCoralBlockItem as ItemRender>::RENDER;

pub struct ItemDeadHornCoralBlockMod;

impl ItemDeadHornCoralBlockMod {
    pub fn init(_block: &mut block_dead_horn_coral_block::BlockDeadHornCoralBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
