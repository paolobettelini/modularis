use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct FloweringAzaleaLeavesBlockItem;

impl Item for FloweringAzaleaLeavesBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:flowering_azalea_leaves_block",
        label: "Flowering Azalea Leaves",
    };
}

impl ItemRender for FloweringAzaleaLeavesBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-flowering-azalea-leaves-block:item/flowering_azalea_leaves_block"),
    };
}

pub const ITEM_INFO: ItemInfo = FloweringAzaleaLeavesBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <FloweringAzaleaLeavesBlockItem as ItemRender>::RENDER;

pub struct ItemFloweringAzaleaLeavesBlockMod;

impl ItemFloweringAzaleaLeavesBlockMod {
    pub fn init(_block: &mut block_flowering_azalea_leaves::BlockFloweringAzaleaLeavesMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
