use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PaleOakLeavesBlockItem;

impl Item for PaleOakLeavesBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:pale_oak_leaves_block",
        label: "Pale Oak Leaves",
    };
}

impl ItemRender for PaleOakLeavesBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-pale-oak-leaves-block:item/pale_oak_leaves_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PaleOakLeavesBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PaleOakLeavesBlockItem as ItemRender>::RENDER;

pub struct ItemPaleOakLeavesBlockMod;

impl ItemPaleOakLeavesBlockMod {
    pub fn init(_block: &mut block_pale_oak_leaves::BlockPaleOakLeavesMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
