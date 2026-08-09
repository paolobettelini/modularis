use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct EmeraldBlockItem;

impl Item for EmeraldBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:emerald_block",
        label: "Emerald Block",
    };
}

impl ItemRender for EmeraldBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-emerald-block:item/emerald_block"),
    };
}

pub const ITEM_INFO: ItemInfo = EmeraldBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <EmeraldBlockItem as ItemRender>::RENDER;

pub struct ItemEmeraldBlockMod;

impl ItemEmeraldBlockMod {
    pub fn init(_block: &mut block_emerald_block::BlockEmeraldBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
