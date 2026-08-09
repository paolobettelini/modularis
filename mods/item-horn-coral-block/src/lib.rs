use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct HornCoralBlockItem;

impl Item for HornCoralBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:horn_coral_block",
        label: "Horn Coral Block",
    };
}

impl ItemRender for HornCoralBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-horn-coral-block:item/horn_coral_block"),
    };
}

pub const ITEM_INFO: ItemInfo = HornCoralBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <HornCoralBlockItem as ItemRender>::RENDER;

pub struct ItemHornCoralBlockMod;

impl ItemHornCoralBlockMod {
    pub fn init(_block: &mut block_horn_coral_block::BlockHornCoralBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
