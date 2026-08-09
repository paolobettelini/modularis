use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BlackWoolBlockItem;

impl Item for BlackWoolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:black_wool_block",
        label: "Black Wool",
    };
}

impl ItemRender for BlackWoolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-black-wool-block:item/black_wool_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BlackWoolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BlackWoolBlockItem as ItemRender>::RENDER;

pub struct ItemBlackWoolBlockMod;

impl ItemBlackWoolBlockMod {
    pub fn init(_block: &mut block_black_wool::BlockBlackWoolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
