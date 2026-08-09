use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct SulfurBlockItem;

impl Item for SulfurBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:sulfur_block",
        label: "Sulfur",
    };
}

impl ItemRender for SulfurBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-sulfur-block:item/sulfur_block"),
    };
}

pub const ITEM_INFO: ItemInfo = SulfurBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SulfurBlockItem as ItemRender>::RENDER;

pub struct ItemSulfurBlockMod;

impl ItemSulfurBlockMod {
    pub fn init(_block: &mut block_sulfur::BlockSulfurMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
