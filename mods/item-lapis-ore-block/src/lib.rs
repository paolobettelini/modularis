use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LapisOreBlockItem;

impl Item for LapisOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:lapis_ore_block",
        label: "Lapis Ore",
    };
}

impl ItemRender for LapisOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-lapis-ore-block:item/lapis_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LapisOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <LapisOreBlockItem as ItemRender>::RENDER;

pub struct ItemLapisOreBlockMod;

impl ItemLapisOreBlockMod {
    pub fn init(_block: &mut block_lapis_ore::BlockLapisOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
