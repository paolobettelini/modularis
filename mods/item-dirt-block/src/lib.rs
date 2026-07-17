use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DirtBlockItem;

impl Item for DirtBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:dirt_block",
        label: "Dirt",
    };
}

impl ItemRender for DirtBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-dirt-block:item/dirt_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DirtBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DirtBlockItem as ItemRender>::RENDER;

pub struct ItemDirtBlockMod;

impl ItemDirtBlockMod {
    pub fn init(_block: &mut block_dirt::BlockDirtMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
