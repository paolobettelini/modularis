use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PolishedDeepslateBlockItem;

impl Item for PolishedDeepslateBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:polished_deepslate_block",
        label: "Polished Deepslate",
    };
}

impl ItemRender for PolishedDeepslateBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-polished-deepslate-block:item/polished_deepslate_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PolishedDeepslateBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PolishedDeepslateBlockItem as ItemRender>::RENDER;

pub struct ItemPolishedDeepslateBlockMod;

impl ItemPolishedDeepslateBlockMod {
    pub fn init(_block: &mut block_polished_deepslate::BlockPolishedDeepslateMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
