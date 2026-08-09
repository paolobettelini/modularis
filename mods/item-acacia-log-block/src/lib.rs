use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct AcaciaLogBlockItem;

impl Item for AcaciaLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:acacia_log_block",
        label: "Acacia Log",
    };
}

impl ItemRender for AcaciaLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-acacia-log-block:item/acacia_log_block"),
    };
}

pub const ITEM_INFO: ItemInfo = AcaciaLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <AcaciaLogBlockItem as ItemRender>::RENDER;

pub struct ItemAcaciaLogBlockMod;

impl ItemAcaciaLogBlockMod {
    pub fn init(_block: &mut block_acacia_log::BlockAcaciaLogMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
