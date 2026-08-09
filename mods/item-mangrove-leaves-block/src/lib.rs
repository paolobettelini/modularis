use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MangroveLeavesBlockItem;

impl Item for MangroveLeavesBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:mangrove_leaves_block",
        label: "Mangrove Leaves",
    };
}

impl ItemRender for MangroveLeavesBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-mangrove-leaves-block:item/mangrove_leaves_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MangroveLeavesBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MangroveLeavesBlockItem as ItemRender>::RENDER;

pub struct ItemMangroveLeavesBlockMod;

impl ItemMangroveLeavesBlockMod {
    pub fn init(_block: &mut block_mangrove_leaves::BlockMangroveLeavesMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
