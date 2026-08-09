use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct AzaleaLeavesBlockItem;

impl Item for AzaleaLeavesBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:azalea_leaves_block",
        label: "Azalea Leaves",
    };
}

impl ItemRender for AzaleaLeavesBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-azalea-leaves-block:item/azalea_leaves_block"),
    };
}

pub const ITEM_INFO: ItemInfo = AzaleaLeavesBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <AzaleaLeavesBlockItem as ItemRender>::RENDER;

pub struct ItemAzaleaLeavesBlockMod;

impl ItemAzaleaLeavesBlockMod {
    pub fn init(_block: &mut block_azalea_leaves::BlockAzaleaLeavesMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
