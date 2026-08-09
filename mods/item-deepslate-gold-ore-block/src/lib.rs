use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DeepslateGoldOreBlockItem;

impl Item for DeepslateGoldOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:deepslate_gold_ore_block",
        label: "Deepslate Gold Ore",
    };
}

impl ItemRender for DeepslateGoldOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-deepslate-gold-ore-block:item/deepslate_gold_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DeepslateGoldOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DeepslateGoldOreBlockItem as ItemRender>::RENDER;

pub struct ItemDeepslateGoldOreBlockMod;

impl ItemDeepslateGoldOreBlockMod {
    pub fn init(_block: &mut block_deepslate_gold_ore::BlockDeepslateGoldOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
