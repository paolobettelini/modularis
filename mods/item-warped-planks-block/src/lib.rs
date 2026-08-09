use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct WarpedPlanksBlockItem;

impl Item for WarpedPlanksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:warped_planks_block",
        label: "Warped Planks",
    };
}

impl ItemRender for WarpedPlanksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-warped-planks-block:item/warped_planks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = WarpedPlanksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <WarpedPlanksBlockItem as ItemRender>::RENDER;

pub struct ItemWarpedPlanksBlockMod;

impl ItemWarpedPlanksBlockMod {
    pub fn init(_block: &mut block_warped_planks::BlockWarpedPlanksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
