use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PolishedCinnabarBlockItem;

impl Item for PolishedCinnabarBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:polished_cinnabar_block",
        label: "Polished Cinnabar",
    };
}

impl ItemRender for PolishedCinnabarBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-polished-cinnabar-block:item/polished_cinnabar_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PolishedCinnabarBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PolishedCinnabarBlockItem as ItemRender>::RENDER;

pub struct ItemPolishedCinnabarBlockMod;

impl ItemPolishedCinnabarBlockMod {
    pub fn init(_block: &mut block_polished_cinnabar::BlockPolishedCinnabarMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
