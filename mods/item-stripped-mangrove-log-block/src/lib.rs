use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct StrippedMangroveLogBlockItem;

impl Item for StrippedMangroveLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:stripped_mangrove_log_block",
        label: "Stripped Mangrove Log",
    };
}

impl ItemRender for StrippedMangroveLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-stripped-mangrove-log-block:item/stripped_mangrove_log_block"),
    };
}

pub const ITEM_INFO: ItemInfo = StrippedMangroveLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <StrippedMangroveLogBlockItem as ItemRender>::RENDER;

pub struct ItemStrippedMangroveLogBlockMod;

impl ItemStrippedMangroveLogBlockMod {
    pub fn init(_block: &mut block_stripped_mangrove_log::BlockStrippedMangroveLogMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
