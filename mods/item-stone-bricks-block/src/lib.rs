use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct StoneBricksBlockItem;

impl Item for StoneBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:stone_bricks_block",
        label: "Stone Bricks",
    };
}

impl ItemRender for StoneBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-stone-bricks-block:item/stone_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = StoneBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <StoneBricksBlockItem as ItemRender>::RENDER;

pub struct ItemStoneBricksBlockMod;

impl ItemStoneBricksBlockMod {
    pub fn init(_block: &mut block_stone_bricks::BlockStoneBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
