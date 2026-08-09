use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DeepslateDiamondOreBlockItem;

impl Item for DeepslateDiamondOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:deepslate_diamond_ore_block",
        label: "Deepslate Diamond Ore",
    };
}

impl ItemRender for DeepslateDiamondOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-deepslate-diamond-ore-block:item/deepslate_diamond_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DeepslateDiamondOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DeepslateDiamondOreBlockItem as ItemRender>::RENDER;

pub struct ItemDeepslateDiamondOreBlockMod;

impl ItemDeepslateDiamondOreBlockMod {
    pub fn init(_block: &mut block_deepslate_diamond_ore::BlockDeepslateDiamondOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
