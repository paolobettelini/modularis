use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct StrippedDarkOakLogBlockItem;

impl Item for StrippedDarkOakLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:stripped_dark_oak_log_block",
        label: "Stripped Dark Oak Log",
    };
}

impl ItemRender for StrippedDarkOakLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-stripped-dark-oak-log-block:item/stripped_dark_oak_log_block"),
    };
}

pub const ITEM_INFO: ItemInfo = StrippedDarkOakLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <StrippedDarkOakLogBlockItem as ItemRender>::RENDER;

pub struct ItemStrippedDarkOakLogBlockMod;

impl ItemStrippedDarkOakLogBlockMod {
    pub fn init(_block: &mut block_stripped_dark_oak_log::BlockStrippedDarkOakLogMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
