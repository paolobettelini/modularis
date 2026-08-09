use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct TubeCoralBlockItem;

impl Item for TubeCoralBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:tube_coral_block",
        label: "Tube Coral Block",
    };
}

impl ItemRender for TubeCoralBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-tube-coral-block:item/tube_coral_block"),
    };
}

pub const ITEM_INFO: ItemInfo = TubeCoralBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <TubeCoralBlockItem as ItemRender>::RENDER;

pub struct ItemTubeCoralBlockMod;

impl ItemTubeCoralBlockMod {
    pub fn init(_block: &mut block_tube_coral_block::BlockTubeCoralBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
