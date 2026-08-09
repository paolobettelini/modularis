use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ChiseledCinnabarBlockItem;

impl Item for ChiseledCinnabarBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:chiseled_cinnabar_block",
        label: "Chiseled Cinnabar",
    };
}

impl ItemRender for ChiseledCinnabarBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-chiseled-cinnabar-block:item/chiseled_cinnabar_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ChiseledCinnabarBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ChiseledCinnabarBlockItem as ItemRender>::RENDER;

pub struct ItemChiseledCinnabarBlockMod;

impl ItemChiseledCinnabarBlockMod {
    pub fn init(_block: &mut block_chiseled_cinnabar::BlockChiseledCinnabarMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
