use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CherryLogBlockItem;

impl Item for CherryLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cherry_log_block",
        label: "Cherry Log",
    };
}

impl ItemRender for CherryLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cherry-log-block:item/cherry_log_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CherryLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CherryLogBlockItem as ItemRender>::RENDER;

pub struct ItemCherryLogBlockMod;

impl ItemCherryLogBlockMod {
    pub fn init(_block: &mut block_cherry_log::BlockCherryLogMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
