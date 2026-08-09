use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BricksBlockItem;

impl Item for BricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:bricks_block",
        label: "Bricks",
    };
}

impl ItemRender for BricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-bricks-block:item/bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BricksBlockItem as ItemRender>::RENDER;

pub struct ItemBricksBlockMod;

impl ItemBricksBlockMod {
    pub fn init(_block: &mut block_bricks::BlockBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
