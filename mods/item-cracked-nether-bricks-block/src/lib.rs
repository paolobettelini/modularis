use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CrackedNetherBricksBlockItem;

impl Item for CrackedNetherBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cracked_nether_bricks_block",
        label: "Cracked Nether Bricks",
    };
}

impl ItemRender for CrackedNetherBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cracked-nether-bricks-block:item/cracked_nether_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CrackedNetherBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CrackedNetherBricksBlockItem as ItemRender>::RENDER;

pub struct ItemCrackedNetherBricksBlockMod;

impl ItemCrackedNetherBricksBlockMod {
    pub fn init(_block: &mut block_cracked_nether_bricks::BlockCrackedNetherBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
