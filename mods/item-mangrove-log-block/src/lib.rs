use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MangroveLogBlockItem;

impl Item for MangroveLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:mangrove_log_block",
        label: "Mangrove Log",
    };
}

impl ItemRender for MangroveLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-mangrove-log-block:item/mangrove_log_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MangroveLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MangroveLogBlockItem as ItemRender>::RENDER;

pub struct ItemMangroveLogBlockMod;

impl ItemMangroveLogBlockMod {
    pub fn init(_block: &mut block_mangrove_log::BlockMangroveLogMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
