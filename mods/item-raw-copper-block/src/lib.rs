use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct RawCopperBlockItem;

impl Item for RawCopperBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:raw_copper_block",
        label: "Raw Copper Block",
    };
}

impl ItemRender for RawCopperBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-raw-copper-block:item/raw_copper_block"),
    };
}

pub const ITEM_INFO: ItemInfo = RawCopperBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <RawCopperBlockItem as ItemRender>::RENDER;

pub struct ItemRawCopperBlockMod;

impl ItemRawCopperBlockMod {
    pub fn init(_block: &mut block_raw_copper_block::BlockRawCopperBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
