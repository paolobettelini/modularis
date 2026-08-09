use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CinnabarBricksBlockItem;

impl Item for CinnabarBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cinnabar_bricks_block",
        label: "Cinnabar Bricks",
    };
}

impl ItemRender for CinnabarBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cinnabar-bricks-block:item/cinnabar_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CinnabarBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CinnabarBricksBlockItem as ItemRender>::RENDER;

pub struct ItemCinnabarBricksBlockMod;

impl ItemCinnabarBricksBlockMod {
    pub fn init(_block: &mut block_cinnabar_bricks::BlockCinnabarBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
