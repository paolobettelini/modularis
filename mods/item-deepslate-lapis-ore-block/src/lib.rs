use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DeepslateLapisOreBlockItem;

impl Item for DeepslateLapisOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:deepslate_lapis_ore_block",
        label: "Deepslate Lapis Ore",
    };
}

impl ItemRender for DeepslateLapisOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-deepslate-lapis-ore-block:item/deepslate_lapis_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DeepslateLapisOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DeepslateLapisOreBlockItem as ItemRender>::RENDER;

pub struct ItemDeepslateLapisOreBlockMod;

impl ItemDeepslateLapisOreBlockMod {
    pub fn init(_block: &mut block_deepslate_lapis_ore::BlockDeepslateLapisOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
