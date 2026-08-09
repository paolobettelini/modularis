use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DarkOakLogBlockItem;

impl Item for DarkOakLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:dark_oak_log_block",
        label: "Dark Oak Log",
    };
}

impl ItemRender for DarkOakLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-dark-oak-log-block:item/dark_oak_log_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DarkOakLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DarkOakLogBlockItem as ItemRender>::RENDER;

pub struct ItemDarkOakLogBlockMod;

impl ItemDarkOakLogBlockMod {
    pub fn init(_block: &mut block_dark_oak_log::BlockDarkOakLogMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
