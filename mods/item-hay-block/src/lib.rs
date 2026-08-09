use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct HayBlockItem;

impl Item for HayBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:hay_block",
        label: "Hay Block",
    };
}

impl ItemRender for HayBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-hay-block:item/hay_block"),
    };
}

pub const ITEM_INFO: ItemInfo = HayBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <HayBlockItem as ItemRender>::RENDER;

pub struct ItemHayBlockMod;

impl ItemHayBlockMod {
    pub fn init(_block: &mut block_hay_block::BlockHayBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
