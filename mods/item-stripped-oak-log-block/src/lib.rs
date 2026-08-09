use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct StrippedOakLogBlockItem;

impl Item for StrippedOakLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:stripped_oak_log_block",
        label: "Stripped Oak Log",
    };
}

impl ItemRender for StrippedOakLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-stripped-oak-log-block:item/stripped_oak_log_block"),
    };
}

pub const ITEM_INFO: ItemInfo = StrippedOakLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <StrippedOakLogBlockItem as ItemRender>::RENDER;

pub struct ItemStrippedOakLogBlockMod;

impl ItemStrippedOakLogBlockMod {
    pub fn init(_block: &mut block_stripped_oak_log::BlockStrippedOakLogMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
