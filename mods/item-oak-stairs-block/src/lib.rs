use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct OakStairsBlockItem;

impl Item for OakStairsBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:oak_stairs_block",
        label: "Oak Stairs",
    };
}

impl ItemRender for OakStairsBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-oak-stairs-block:item/oak_stairs_block"),
    };
}

pub const ITEM_INFO: ItemInfo = OakStairsBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <OakStairsBlockItem as ItemRender>::RENDER;

pub struct ItemOakStairsBlockMod;

impl ItemOakStairsBlockMod {
    pub fn init(_block: &mut block_oak_stairs::BlockOakStairsMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
