use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DeepslateIronOreBlockItem;

impl Item for DeepslateIronOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:deepslate_iron_ore_block",
        label: "Deepslate Iron Ore",
    };
}

impl ItemRender for DeepslateIronOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-deepslate-iron-ore-block:item/deepslate_iron_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DeepslateIronOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DeepslateIronOreBlockItem as ItemRender>::RENDER;

pub struct ItemDeepslateIronOreBlockMod;

impl ItemDeepslateIronOreBlockMod {
    pub fn init(_block: &mut block_deepslate_iron_ore::BlockDeepslateIronOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
