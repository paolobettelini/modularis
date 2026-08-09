use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct AcaciaLeavesBlockItem;

impl Item for AcaciaLeavesBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:acacia_leaves_block",
        label: "Acacia Leaves",
    };
}

impl ItemRender for AcaciaLeavesBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-acacia-leaves-block:item/acacia_leaves_block"),
    };
}

pub const ITEM_INFO: ItemInfo = AcaciaLeavesBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <AcaciaLeavesBlockItem as ItemRender>::RENDER;

pub struct ItemAcaciaLeavesBlockMod;

impl ItemAcaciaLeavesBlockMod {
    pub fn init(_block: &mut block_acacia_leaves::BlockAcaciaLeavesMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
