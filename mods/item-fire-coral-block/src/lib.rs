use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct FireCoralBlockItem;

impl Item for FireCoralBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:fire_coral_block",
        label: "Fire Coral Block",
    };
}

impl ItemRender for FireCoralBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-fire-coral-block:item/fire_coral_block"),
    };
}

pub const ITEM_INFO: ItemInfo = FireCoralBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <FireCoralBlockItem as ItemRender>::RENDER;

pub struct ItemFireCoralBlockMod;

impl ItemFireCoralBlockMod {
    pub fn init(_block: &mut block_fire_coral_block::BlockFireCoralBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
