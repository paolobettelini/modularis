use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct AndesiteBlockItem;

impl Item for AndesiteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:andesite_block",
        label: "Andesite",
    };
}

impl ItemRender for AndesiteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-andesite-block:item/andesite_block"),
    };
}

pub const ITEM_INFO: ItemInfo = AndesiteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <AndesiteBlockItem as ItemRender>::RENDER;

pub struct ItemAndesiteBlockMod;

impl ItemAndesiteBlockMod {
    pub fn init(_block: &mut block_andesite::BlockAndesiteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
