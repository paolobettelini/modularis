use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct StrippedAcaciaLogBlockItem;

impl Item for StrippedAcaciaLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:stripped_acacia_log_block",
        label: "Stripped Acacia Log",
    };
}

impl ItemRender for StrippedAcaciaLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-stripped-acacia-log-block:item/stripped_acacia_log_block"),
    };
}

pub const ITEM_INFO: ItemInfo = StrippedAcaciaLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <StrippedAcaciaLogBlockItem as ItemRender>::RENDER;

pub struct ItemStrippedAcaciaLogBlockMod;

impl ItemStrippedAcaciaLogBlockMod {
    pub fn init(_block: &mut block_stripped_acacia_log::BlockStrippedAcaciaLogMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
