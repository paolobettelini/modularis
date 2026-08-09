use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CherryPlanksBlockItem;

impl Item for CherryPlanksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cherry_planks_block",
        label: "Cherry Planks",
    };
}

impl ItemRender for CherryPlanksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cherry-planks-block:item/cherry_planks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CherryPlanksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CherryPlanksBlockItem as ItemRender>::RENDER;

pub struct ItemCherryPlanksBlockMod;

impl ItemCherryPlanksBlockMod {
    pub fn init(_block: &mut block_cherry_planks::BlockCherryPlanksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
