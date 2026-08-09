use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CherryLeavesBlockItem;

impl Item for CherryLeavesBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cherry_leaves_block",
        label: "Cherry Leaves",
    };
}

impl ItemRender for CherryLeavesBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cherry-leaves-block:item/cherry_leaves_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CherryLeavesBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CherryLeavesBlockItem as ItemRender>::RENDER;

pub struct ItemCherryLeavesBlockMod;

impl ItemCherryLeavesBlockMod {
    pub fn init(_block: &mut block_cherry_leaves::BlockCherryLeavesMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
