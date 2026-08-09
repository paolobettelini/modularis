use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct RawGoldBlockItem;

impl Item for RawGoldBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:raw_gold_block",
        label: "Raw Gold Block",
    };
}

impl ItemRender for RawGoldBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-raw-gold-block:item/raw_gold_block"),
    };
}

pub const ITEM_INFO: ItemInfo = RawGoldBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <RawGoldBlockItem as ItemRender>::RENDER;

pub struct ItemRawGoldBlockMod;

impl ItemRawGoldBlockMod {
    pub fn init(_block: &mut block_raw_gold_block::BlockRawGoldBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
