use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ChiseledTuffBlockItem;

impl Item for ChiseledTuffBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:chiseled_tuff_block",
        label: "Chiseled Tuff",
    };
}

impl ItemRender for ChiseledTuffBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-chiseled-tuff-block:item/chiseled_tuff_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ChiseledTuffBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ChiseledTuffBlockItem as ItemRender>::RENDER;

pub struct ItemChiseledTuffBlockMod;

impl ItemChiseledTuffBlockMod {
    pub fn init(_block: &mut block_chiseled_tuff::BlockChiseledTuffMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
