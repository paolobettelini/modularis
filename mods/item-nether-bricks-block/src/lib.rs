use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct NetherBricksBlockItem;

impl Item for NetherBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:nether_bricks_block",
        label: "Nether Bricks",
    };
}

impl ItemRender for NetherBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-nether-bricks-block:item/nether_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = NetherBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <NetherBricksBlockItem as ItemRender>::RENDER;

pub struct ItemNetherBricksBlockMod;

impl ItemNetherBricksBlockMod {
    pub fn init(_block: &mut block_nether_bricks::BlockNetherBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
