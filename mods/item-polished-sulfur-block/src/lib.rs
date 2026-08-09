use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PolishedSulfurBlockItem;

impl Item for PolishedSulfurBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:polished_sulfur_block",
        label: "Polished Sulfur",
    };
}

impl ItemRender for PolishedSulfurBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-polished-sulfur-block:item/polished_sulfur_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PolishedSulfurBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PolishedSulfurBlockItem as ItemRender>::RENDER;

pub struct ItemPolishedSulfurBlockMod;

impl ItemPolishedSulfurBlockMod {
    pub fn init(_block: &mut block_polished_sulfur::BlockPolishedSulfurMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
