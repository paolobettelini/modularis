use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LapisBlockItem;

impl Item for LapisBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:lapis_block",
        label: "Lapis Block",
    };
}

impl ItemRender for LapisBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-lapis-block:item/lapis_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LapisBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <LapisBlockItem as ItemRender>::RENDER;

pub struct ItemLapisBlockMod;

impl ItemLapisBlockMod {
    pub fn init(_block: &mut block_lapis_block::BlockLapisBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
