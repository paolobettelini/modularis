use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DeepslateBlockItem;

impl Item for DeepslateBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:deepslate_block",
        label: "Deepslate",
    };
}

impl ItemRender for DeepslateBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-deepslate-block:item/deepslate_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DeepslateBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DeepslateBlockItem as ItemRender>::RENDER;

pub struct ItemDeepslateBlockMod;

impl ItemDeepslateBlockMod {
    pub fn init(_block: &mut block_deepslate::BlockDeepslateMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
