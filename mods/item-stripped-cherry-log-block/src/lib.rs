use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct StrippedCherryLogBlockItem;

impl Item for StrippedCherryLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:stripped_cherry_log_block",
        label: "Stripped Cherry Log",
    };
}

impl ItemRender for StrippedCherryLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-stripped-cherry-log-block:item/stripped_cherry_log_block"),
    };
}

pub const ITEM_INFO: ItemInfo = StrippedCherryLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <StrippedCherryLogBlockItem as ItemRender>::RENDER;

pub struct ItemStrippedCherryLogBlockMod;

impl ItemStrippedCherryLogBlockMod {
    pub fn init(_block: &mut block_stripped_cherry_log::BlockStrippedCherryLogMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
