use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DarkOakPlanksBlockItem;

impl Item for DarkOakPlanksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:dark_oak_planks_block",
        label: "Dark Oak Planks",
    };
}

impl ItemRender for DarkOakPlanksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-dark-oak-planks-block:item/dark_oak_planks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DarkOakPlanksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DarkOakPlanksBlockItem as ItemRender>::RENDER;

pub struct ItemDarkOakPlanksBlockMod;

impl ItemDarkOakPlanksBlockMod {
    pub fn init(_block: &mut block_dark_oak_planks::BlockDarkOakPlanksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
