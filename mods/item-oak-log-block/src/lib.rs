use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct OakLogBlockItem;
impl Item for OakLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:oak_log_block",
        label: "Oak Log",
    };
}

impl ItemRender for OakLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-oak-log-block:item/oak_log_block"),
    };
}
pub const ITEM_INFO: ItemInfo = OakLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <OakLogBlockItem as ItemRender>::RENDER;
pub struct ItemOakLogBlockMod;
impl ItemOakLogBlockMod {
    pub fn init(_block: &mut block_oak_log::BlockOakLogMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
