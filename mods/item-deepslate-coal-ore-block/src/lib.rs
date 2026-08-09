use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DeepslateCoalOreBlockItem;

impl Item for DeepslateCoalOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:deepslate_coal_ore_block",
        label: "Deepslate Coal Ore",
    };
}

impl ItemRender for DeepslateCoalOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-deepslate-coal-ore-block:item/deepslate_coal_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DeepslateCoalOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DeepslateCoalOreBlockItem as ItemRender>::RENDER;

pub struct ItemDeepslateCoalOreBlockMod;

impl ItemDeepslateCoalOreBlockMod {
    pub fn init(_block: &mut block_deepslate_coal_ore::BlockDeepslateCoalOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
