use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CopperOreBlockItem;

impl Item for CopperOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:copper_ore_block",
        label: "Copper Ore",
    };
}

impl ItemRender for CopperOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-copper-ore-block:item/copper_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CopperOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CopperOreBlockItem as ItemRender>::RENDER;

pub struct ItemCopperOreBlockMod;

impl ItemCopperOreBlockMod {
    pub fn init(_block: &mut block_copper_ore::BlockCopperOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
