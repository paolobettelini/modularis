use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ChiseledStoneBricksBlockItem;

impl Item for ChiseledStoneBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:chiseled_stone_bricks_block",
        label: "Chiseled Stone Bricks",
    };
}

impl ItemRender for ChiseledStoneBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-chiseled-stone-bricks-block:item/chiseled_stone_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ChiseledStoneBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ChiseledStoneBricksBlockItem as ItemRender>::RENDER;

pub struct ItemChiseledStoneBricksBlockMod;

impl ItemChiseledStoneBricksBlockMod {
    pub fn init(_block: &mut block_chiseled_stone_bricks::BlockChiseledStoneBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
