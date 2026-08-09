use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct WhiteWoolBlockItem;

impl Item for WhiteWoolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:white_wool_block",
        label: "White Wool",
    };
}

impl ItemRender for WhiteWoolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-white-wool-block:item/white_wool_block"),
    };
}

pub const ITEM_INFO: ItemInfo = WhiteWoolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <WhiteWoolBlockItem as ItemRender>::RENDER;

pub struct ItemWhiteWoolBlockMod;

impl ItemWhiteWoolBlockMod {
    pub fn init(_block: &mut block_white_wool::BlockWhiteWoolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
