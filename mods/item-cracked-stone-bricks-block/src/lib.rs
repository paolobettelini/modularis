use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CrackedStoneBricksBlockItem;

impl Item for CrackedStoneBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cracked_stone_bricks_block",
        label: "Cracked Stone Bricks",
    };
}

impl ItemRender for CrackedStoneBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cracked-stone-bricks-block:item/cracked_stone_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CrackedStoneBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CrackedStoneBricksBlockItem as ItemRender>::RENDER;

pub struct ItemCrackedStoneBricksBlockMod;

impl ItemCrackedStoneBricksBlockMod {
    pub fn init(_block: &mut block_cracked_stone_bricks::BlockCrackedStoneBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
