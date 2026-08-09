use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct TuffBlockItem;

impl Item for TuffBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:tuff_block",
        label: "Tuff",
    };
}

impl ItemRender for TuffBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-tuff-block:item/tuff_block"),
    };
}

pub const ITEM_INFO: ItemInfo = TuffBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <TuffBlockItem as ItemRender>::RENDER;

pub struct ItemTuffBlockMod;

impl ItemTuffBlockMod {
    pub fn init(_block: &mut block_tuff::BlockTuffMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
