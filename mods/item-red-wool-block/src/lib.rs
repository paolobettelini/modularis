use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct RedWoolBlockItem;

impl Item for RedWoolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:red_wool_block",
        label: "Red Wool",
    };
}

impl ItemRender for RedWoolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-red-wool-block:item/red_wool_block"),
    };
}

pub const ITEM_INFO: ItemInfo = RedWoolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <RedWoolBlockItem as ItemRender>::RENDER;

pub struct ItemRedWoolBlockMod;

impl ItemRedWoolBlockMod {
    pub fn init(_block: &mut block_red_wool::BlockRedWoolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
