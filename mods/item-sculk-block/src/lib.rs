use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct SculkBlockItem;

impl Item for SculkBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:sculk_block",
        label: "Sculk",
    };
}

impl ItemRender for SculkBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-sculk-block:item/sculk_block"),
    };
}

pub const ITEM_INFO: ItemInfo = SculkBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SculkBlockItem as ItemRender>::RENDER;

pub struct ItemSculkBlockMod;

impl ItemSculkBlockMod {
    pub fn init(_block: &mut block_sculk::BlockSculkMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
