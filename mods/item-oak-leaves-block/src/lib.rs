use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct OakLeavesBlockItem;
impl Item for OakLeavesBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:oak_leaves_block",
        label: "Oak Leaves",
    };
}

impl ItemRender for OakLeavesBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-oak-leaves-block:item/oak_leaves_block"),
    };
}
pub const ITEM_INFO: ItemInfo = OakLeavesBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <OakLeavesBlockItem as ItemRender>::RENDER;
pub struct ItemOakLeavesBlockMod;
impl ItemOakLeavesBlockMod {
    pub fn init(_block: &mut block_oak_leaves::BlockOakLeavesMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
