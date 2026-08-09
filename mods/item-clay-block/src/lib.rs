use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ClayBlockItem;

impl Item for ClayBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:clay_block",
        label: "Clay",
    };
}

impl ItemRender for ClayBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-clay-block:item/clay_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ClayBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ClayBlockItem as ItemRender>::RENDER;

pub struct ItemClayBlockMod;

impl ItemClayBlockMod {
    pub fn init(_block: &mut block_clay::BlockClayMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
