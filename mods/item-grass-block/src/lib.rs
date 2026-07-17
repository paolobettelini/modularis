use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GrassBlockItem;

impl Item for GrassBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:grass_block",
        label: "Grass",
    };
}

impl ItemRender for GrassBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-grass-block:item/grass_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GrassBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GrassBlockItem as ItemRender>::RENDER;

pub struct ItemGrassBlockMod;

impl ItemGrassBlockMod {
    pub fn init(_block: &mut block_grass::BlockGrassMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
