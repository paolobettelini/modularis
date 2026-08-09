use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct RedNetherBricksBlockItem;

impl Item for RedNetherBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:red_nether_bricks_block",
        label: "Red Nether Bricks",
    };
}

impl ItemRender for RedNetherBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-red-nether-bricks-block:item/red_nether_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = RedNetherBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <RedNetherBricksBlockItem as ItemRender>::RENDER;

pub struct ItemRedNetherBricksBlockMod;

impl ItemRedNetherBricksBlockMod {
    pub fn init(_block: &mut block_red_nether_bricks::BlockRedNetherBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
