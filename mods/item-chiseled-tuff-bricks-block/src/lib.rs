use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ChiseledTuffBricksBlockItem;

impl Item for ChiseledTuffBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:chiseled_tuff_bricks_block",
        label: "Chiseled Tuff Bricks",
    };
}

impl ItemRender for ChiseledTuffBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-chiseled-tuff-bricks-block:item/chiseled_tuff_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ChiseledTuffBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ChiseledTuffBricksBlockItem as ItemRender>::RENDER;

pub struct ItemChiseledTuffBricksBlockMod;

impl ItemChiseledTuffBricksBlockMod {
    pub fn init(_block: &mut block_chiseled_tuff_bricks::BlockChiseledTuffBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
