use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PolishedGraniteBlockItem;

impl Item for PolishedGraniteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:polished_granite_block",
        label: "Polished Granite",
    };
}

impl ItemRender for PolishedGraniteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-polished-granite-block:item/polished_granite_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PolishedGraniteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PolishedGraniteBlockItem as ItemRender>::RENDER;

pub struct ItemPolishedGraniteBlockMod;

impl ItemPolishedGraniteBlockMod {
    pub fn init(_block: &mut block_polished_granite::BlockPolishedGraniteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
