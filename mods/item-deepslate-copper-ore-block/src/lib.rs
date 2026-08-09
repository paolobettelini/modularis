use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DeepslateCopperOreBlockItem;

impl Item for DeepslateCopperOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:deepslate_copper_ore_block",
        label: "Deepslate Copper Ore",
    };
}

impl ItemRender for DeepslateCopperOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-deepslate-copper-ore-block:item/deepslate_copper_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DeepslateCopperOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DeepslateCopperOreBlockItem as ItemRender>::RENDER;

pub struct ItemDeepslateCopperOreBlockMod;

impl ItemDeepslateCopperOreBlockMod {
    pub fn init(_block: &mut block_deepslate_copper_ore::BlockDeepslateCopperOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
