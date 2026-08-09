use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ReinforcedDeepslateBlockItem;

impl Item for ReinforcedDeepslateBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:reinforced_deepslate_block",
        label: "Reinforced Deepslate",
    };
}

impl ItemRender for ReinforcedDeepslateBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-reinforced-deepslate-block:item/reinforced_deepslate_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ReinforcedDeepslateBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ReinforcedDeepslateBlockItem as ItemRender>::RENDER;

pub struct ItemReinforcedDeepslateBlockMod;

impl ItemReinforcedDeepslateBlockMod {
    pub fn init(_block: &mut block_reinforced_deepslate::BlockReinforcedDeepslateMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
