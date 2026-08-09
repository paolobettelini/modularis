use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PaleOakLogBlockItem;

impl Item for PaleOakLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:pale_oak_log_block",
        label: "Pale Oak Log",
    };
}

impl ItemRender for PaleOakLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-pale-oak-log-block:item/pale_oak_log_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PaleOakLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PaleOakLogBlockItem as ItemRender>::RENDER;

pub struct ItemPaleOakLogBlockMod;

impl ItemPaleOakLogBlockMod {
    pub fn init(_block: &mut block_pale_oak_log::BlockPaleOakLogMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
