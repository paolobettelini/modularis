use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CyanWoolBlockItem;

impl Item for CyanWoolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cyan_wool_block",
        label: "Cyan Wool",
    };
}

impl ItemRender for CyanWoolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cyan-wool-block:item/cyan_wool_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CyanWoolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CyanWoolBlockItem as ItemRender>::RENDER;

pub struct ItemCyanWoolBlockMod;

impl ItemCyanWoolBlockMod {
    pub fn init(_block: &mut block_cyan_wool::BlockCyanWoolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
