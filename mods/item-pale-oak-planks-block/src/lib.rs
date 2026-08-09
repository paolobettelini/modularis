use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PaleOakPlanksBlockItem;

impl Item for PaleOakPlanksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:pale_oak_planks_block",
        label: "Pale Oak Planks",
    };
}

impl ItemRender for PaleOakPlanksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-pale-oak-planks-block:item/pale_oak_planks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PaleOakPlanksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PaleOakPlanksBlockItem as ItemRender>::RENDER;

pub struct ItemPaleOakPlanksBlockMod;

impl ItemPaleOakPlanksBlockMod {
    pub fn init(_block: &mut block_pale_oak_planks::BlockPaleOakPlanksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
