use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DeepslateEmeraldOreBlockItem;

impl Item for DeepslateEmeraldOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:deepslate_emerald_ore_block",
        label: "Deepslate Emerald Ore",
    };
}

impl ItemRender for DeepslateEmeraldOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-deepslate-emerald-ore-block:item/deepslate_emerald_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DeepslateEmeraldOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DeepslateEmeraldOreBlockItem as ItemRender>::RENDER;

pub struct ItemDeepslateEmeraldOreBlockMod;

impl ItemDeepslateEmeraldOreBlockMod {
    pub fn init(_block: &mut block_deepslate_emerald_ore::BlockDeepslateEmeraldOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
