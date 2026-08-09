use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct OakPlanksBlockItem;

impl Item for OakPlanksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:oak_planks_block",
        label: "Oak Planks",
    };
}

impl ItemRender for OakPlanksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-oak-planks-block:item/oak_planks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = OakPlanksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <OakPlanksBlockItem as ItemRender>::RENDER;

pub struct ItemOakPlanksBlockMod;

impl ItemOakPlanksBlockMod {
    pub fn init(_block: &mut block_oak_planks::BlockOakPlanksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
