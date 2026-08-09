use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CobbledDeepslateBlockItem;

impl Item for CobbledDeepslateBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cobbled_deepslate_block",
        label: "Cobbled Deepslate",
    };
}

impl ItemRender for CobbledDeepslateBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cobbled-deepslate-block:item/cobbled_deepslate_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CobbledDeepslateBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CobbledDeepslateBlockItem as ItemRender>::RENDER;

pub struct ItemCobbledDeepslateBlockMod;

impl ItemCobbledDeepslateBlockMod {
    pub fn init(_block: &mut block_cobbled_deepslate::BlockCobbledDeepslateMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
