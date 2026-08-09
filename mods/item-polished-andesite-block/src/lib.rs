use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PolishedAndesiteBlockItem;

impl Item for PolishedAndesiteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:polished_andesite_block",
        label: "Polished Andesite",
    };
}

impl ItemRender for PolishedAndesiteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-polished-andesite-block:item/polished_andesite_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PolishedAndesiteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PolishedAndesiteBlockItem as ItemRender>::RENDER;

pub struct ItemPolishedAndesiteBlockMod;

impl ItemPolishedAndesiteBlockMod {
    pub fn init(_block: &mut block_polished_andesite::BlockPolishedAndesiteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
