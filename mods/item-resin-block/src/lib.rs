use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ResinBlockItem;

impl Item for ResinBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:resin_block",
        label: "Resin Block",
    };
}

impl ItemRender for ResinBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-resin-block:item/resin_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ResinBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ResinBlockItem as ItemRender>::RENDER;

pub struct ItemResinBlockMod;

impl ItemResinBlockMod {
    pub fn init(_block: &mut block_resin_block::BlockResinBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
