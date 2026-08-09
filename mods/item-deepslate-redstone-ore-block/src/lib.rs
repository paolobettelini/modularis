use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DeepslateRedstoneOreBlockItem;

impl Item for DeepslateRedstoneOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:deepslate_redstone_ore_block",
        label: "Deepslate Redstone Ore",
    };
}

impl ItemRender for DeepslateRedstoneOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-deepslate-redstone-ore-block:item/deepslate_redstone_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DeepslateRedstoneOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DeepslateRedstoneOreBlockItem as ItemRender>::RENDER;

pub struct ItemDeepslateRedstoneOreBlockMod;

impl ItemDeepslateRedstoneOreBlockMod {
    pub fn init(_block: &mut block_deepslate_redstone_ore::BlockDeepslateRedstoneOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
