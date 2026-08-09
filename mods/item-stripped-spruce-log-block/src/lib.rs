use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct StrippedSpruceLogBlockItem;

impl Item for StrippedSpruceLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:stripped_spruce_log_block",
        label: "Stripped Spruce Log",
    };
}

impl ItemRender for StrippedSpruceLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-stripped-spruce-log-block:item/stripped_spruce_log_block"),
    };
}

pub const ITEM_INFO: ItemInfo = StrippedSpruceLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <StrippedSpruceLogBlockItem as ItemRender>::RENDER;

pub struct ItemStrippedSpruceLogBlockMod;

impl ItemStrippedSpruceLogBlockMod {
    pub fn init(_block: &mut block_stripped_spruce_log::BlockStrippedSpruceLogMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
