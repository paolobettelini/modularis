use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CinnabarBlockItem;

impl Item for CinnabarBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cinnabar_block",
        label: "Cinnabar",
    };
}

impl ItemRender for CinnabarBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cinnabar-block:item/cinnabar_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CinnabarBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CinnabarBlockItem as ItemRender>::RENDER;

pub struct ItemCinnabarBlockMod;

impl ItemCinnabarBlockMod {
    pub fn init(_block: &mut block_cinnabar::BlockCinnabarMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
