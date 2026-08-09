use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct RawIronBlockItem;

impl Item for RawIronBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:raw_iron_block",
        label: "Raw Iron Block",
    };
}

impl ItemRender for RawIronBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-raw-iron-block:item/raw_iron_block"),
    };
}

pub const ITEM_INFO: ItemInfo = RawIronBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <RawIronBlockItem as ItemRender>::RENDER;

pub struct ItemRawIronBlockMod;

impl ItemRawIronBlockMod {
    pub fn init(_block: &mut block_raw_iron_block::BlockRawIronBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
