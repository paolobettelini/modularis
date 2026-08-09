use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LimeWoolBlockItem;

impl Item for LimeWoolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:lime_wool_block",
        label: "Lime Wool",
    };
}

impl ItemRender for LimeWoolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-lime-wool-block:item/lime_wool_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LimeWoolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <LimeWoolBlockItem as ItemRender>::RENDER;

pub struct ItemLimeWoolBlockMod;

impl ItemLimeWoolBlockMod {
    pub fn init(_block: &mut block_lime_wool::BlockLimeWoolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
