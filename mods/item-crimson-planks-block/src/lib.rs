use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CrimsonPlanksBlockItem;

impl Item for CrimsonPlanksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:crimson_planks_block",
        label: "Crimson Planks",
    };
}

impl ItemRender for CrimsonPlanksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-crimson-planks-block:item/crimson_planks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CrimsonPlanksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CrimsonPlanksBlockItem as ItemRender>::RENDER;

pub struct ItemCrimsonPlanksBlockMod;

impl ItemCrimsonPlanksBlockMod {
    pub fn init(_block: &mut block_crimson_planks::BlockCrimsonPlanksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
