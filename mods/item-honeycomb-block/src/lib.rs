use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct HoneycombBlockItem;

impl Item for HoneycombBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:honeycomb_block",
        label: "Honeycomb Block",
    };
}

impl ItemRender for HoneycombBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-honeycomb-block:item/honeycomb_block"),
    };
}

pub const ITEM_INFO: ItemInfo = HoneycombBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <HoneycombBlockItem as ItemRender>::RENDER;

pub struct ItemHoneycombBlockMod;

impl ItemHoneycombBlockMod {
    pub fn init(_block: &mut block_honeycomb_block::BlockHoneycombBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
