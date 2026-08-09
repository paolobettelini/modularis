use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DeadFireCoralBlockItem;

impl Item for DeadFireCoralBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:dead_fire_coral_block",
        label: "Dead Fire Coral Block",
    };
}

impl ItemRender for DeadFireCoralBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-dead-fire-coral-block:item/dead_fire_coral_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DeadFireCoralBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DeadFireCoralBlockItem as ItemRender>::RENDER;

pub struct ItemDeadFireCoralBlockMod;

impl ItemDeadFireCoralBlockMod {
    pub fn init(_block: &mut block_dead_fire_coral_block::BlockDeadFireCoralBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
