use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ChiseledDeepslateBlockItem;

impl Item for ChiseledDeepslateBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:chiseled_deepslate_block",
        label: "Chiseled Deepslate",
    };
}

impl ItemRender for ChiseledDeepslateBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-chiseled-deepslate-block:item/chiseled_deepslate_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ChiseledDeepslateBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ChiseledDeepslateBlockItem as ItemRender>::RENDER;

pub struct ItemChiseledDeepslateBlockMod;

impl ItemChiseledDeepslateBlockMod {
    pub fn init(_block: &mut block_chiseled_deepslate::BlockChiseledDeepslateMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
