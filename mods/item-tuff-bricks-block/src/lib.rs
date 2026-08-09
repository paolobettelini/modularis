use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct TuffBricksBlockItem;

impl Item for TuffBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:tuff_bricks_block",
        label: "Tuff Bricks",
    };
}

impl ItemRender for TuffBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-tuff-bricks-block:item/tuff_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = TuffBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <TuffBricksBlockItem as ItemRender>::RENDER;

pub struct ItemTuffBricksBlockMod;

impl ItemTuffBricksBlockMod {
    pub fn init(_block: &mut block_tuff_bricks::BlockTuffBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
