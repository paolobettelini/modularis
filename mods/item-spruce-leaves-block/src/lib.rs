use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct SpruceLeavesBlockItem;

impl Item for SpruceLeavesBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:spruce_leaves_block",
        label: "Spruce Leaves",
    };
}

impl ItemRender for SpruceLeavesBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-spruce-leaves-block:item/spruce_leaves_block"),
    };
}

pub const ITEM_INFO: ItemInfo = SpruceLeavesBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SpruceLeavesBlockItem as ItemRender>::RENDER;

pub struct ItemSpruceLeavesBlockMod;

impl ItemSpruceLeavesBlockMod {
    pub fn init(_block: &mut block_spruce_leaves::BlockSpruceLeavesMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
