use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DiamondOreBlockItem;

impl Item for DiamondOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:diamond_ore_block",
        label: "Diamond Ore",
    };
}

impl ItemRender for DiamondOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-diamond-ore-block:item/diamond_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DiamondOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DiamondOreBlockItem as ItemRender>::RENDER;

pub struct ItemDiamondOreBlockMod;

impl ItemDiamondOreBlockMod {
    pub fn init(_block: &mut block_diamond_ore::BlockDiamondOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
