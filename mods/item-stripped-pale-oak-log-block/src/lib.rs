use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct StrippedPaleOakLogBlockItem;

impl Item for StrippedPaleOakLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:stripped_pale_oak_log_block",
        label: "Stripped Pale Oak Log",
    };
}

impl ItemRender for StrippedPaleOakLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-stripped-pale-oak-log-block:item/stripped_pale_oak_log_block"),
    };
}

pub const ITEM_INFO: ItemInfo = StrippedPaleOakLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <StrippedPaleOakLogBlockItem as ItemRender>::RENDER;

pub struct ItemStrippedPaleOakLogBlockMod;

impl ItemStrippedPaleOakLogBlockMod {
    pub fn init(_block: &mut block_stripped_pale_oak_log::BlockStrippedPaleOakLogMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
