use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CopperBlockItem;

impl Item for CopperBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:copper_block",
        label: "Copper Block",
    };
}

impl ItemRender for CopperBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-copper-block:item/copper_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CopperBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CopperBlockItem as ItemRender>::RENDER;

pub struct ItemCopperBlockMod;

impl ItemCopperBlockMod {
    pub fn init(_block: &mut block_copper_block::BlockCopperBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
