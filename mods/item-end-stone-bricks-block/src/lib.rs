use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct EndStoneBricksBlockItem;

impl Item for EndStoneBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:end_stone_bricks_block",
        label: "End Stone Bricks",
    };
}

impl ItemRender for EndStoneBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-end-stone-bricks-block:item/end_stone_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = EndStoneBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <EndStoneBricksBlockItem as ItemRender>::RENDER;

pub struct ItemEndStoneBricksBlockMod;

impl ItemEndStoneBricksBlockMod {
    pub fn init(_block: &mut block_end_stone_bricks::BlockEndStoneBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
