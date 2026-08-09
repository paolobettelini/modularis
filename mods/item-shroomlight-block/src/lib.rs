use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ShroomlightBlockItem;

impl Item for ShroomlightBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:shroomlight_block",
        label: "Shroomlight",
    };
}

impl ItemRender for ShroomlightBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-shroomlight-block:item/shroomlight_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ShroomlightBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ShroomlightBlockItem as ItemRender>::RENDER;

pub struct ItemShroomlightBlockMod;

impl ItemShroomlightBlockMod {
    pub fn init(_block: &mut block_shroomlight::BlockShroomlightMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
