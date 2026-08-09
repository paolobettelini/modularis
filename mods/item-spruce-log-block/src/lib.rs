use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct SpruceLogBlockItem;

impl Item for SpruceLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:spruce_log_block",
        label: "Spruce Log",
    };
}

impl ItemRender for SpruceLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-spruce-log-block:item/spruce_log_block"),
    };
}

pub const ITEM_INFO: ItemInfo = SpruceLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SpruceLogBlockItem as ItemRender>::RENDER;

pub struct ItemSpruceLogBlockMod;

impl ItemSpruceLogBlockMod {
    pub fn init(_block: &mut block_spruce_log::BlockSpruceLogMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
