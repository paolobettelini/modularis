use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CoarseDirtBlockItem;

impl Item for CoarseDirtBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:coarse_dirt_block",
        label: "Coarse Dirt",
    };
}

impl ItemRender for CoarseDirtBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-coarse-dirt-block:item/coarse_dirt_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CoarseDirtBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CoarseDirtBlockItem as ItemRender>::RENDER;

pub struct ItemCoarseDirtBlockMod;

impl ItemCoarseDirtBlockMod {
    pub fn init(_block: &mut block_coarse_dirt::BlockCoarseDirtMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
