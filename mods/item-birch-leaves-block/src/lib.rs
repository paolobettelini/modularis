use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct BirchLeavesBlockItem;
impl Item for BirchLeavesBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:birch_leaves_block",
        label: "Birch Leaves",
    };
}

impl ItemRender for BirchLeavesBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-birch-leaves-block:item/birch_leaves_block"),
    };
}
pub const ITEM_INFO: ItemInfo = BirchLeavesBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BirchLeavesBlockItem as ItemRender>::RENDER;
pub struct ItemBirchLeavesBlockMod;
impl ItemBirchLeavesBlockMod {
    pub fn init(_block: &mut block_birch_leaves::BlockBirchLeavesMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
