use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct StrippedBirchLogBlockItem;

impl Item for StrippedBirchLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:stripped_birch_log_block",
        label: "Stripped Birch Log",
    };
}

impl ItemRender for StrippedBirchLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-stripped-birch-log-block:item/stripped_birch_log_block"),
    };
}

pub const ITEM_INFO: ItemInfo = StrippedBirchLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <StrippedBirchLogBlockItem as ItemRender>::RENDER;

pub struct ItemStrippedBirchLogBlockMod;

impl ItemStrippedBirchLogBlockMod {
    pub fn init(_block: &mut block_stripped_birch_log::BlockStrippedBirchLogMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
