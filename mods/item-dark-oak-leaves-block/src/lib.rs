use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DarkOakLeavesBlockItem;

impl Item for DarkOakLeavesBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:dark_oak_leaves_block",
        label: "Dark Oak Leaves",
    };
}

impl ItemRender for DarkOakLeavesBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-dark-oak-leaves-block:item/dark_oak_leaves_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DarkOakLeavesBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DarkOakLeavesBlockItem as ItemRender>::RENDER;

pub struct ItemDarkOakLeavesBlockMod;

impl ItemDarkOakLeavesBlockMod {
    pub fn init(_block: &mut block_dark_oak_leaves::BlockDarkOakLeavesMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
