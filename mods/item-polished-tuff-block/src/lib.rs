use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PolishedTuffBlockItem;

impl Item for PolishedTuffBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:polished_tuff_block",
        label: "Polished Tuff",
    };
}

impl ItemRender for PolishedTuffBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-polished-tuff-block:item/polished_tuff_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PolishedTuffBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PolishedTuffBlockItem as ItemRender>::RENDER;

pub struct ItemPolishedTuffBlockMod;

impl ItemPolishedTuffBlockMod {
    pub fn init(_block: &mut block_polished_tuff::BlockPolishedTuffMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
