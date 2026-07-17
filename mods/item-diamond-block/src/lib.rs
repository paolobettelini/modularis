use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DiamondBlockItem;

impl Item for DiamondBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:diamond_block",
        label: "Diamond Block",
    };
}

impl ItemRender for DiamondBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-diamond-block:item/diamond_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DiamondBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DiamondBlockItem as ItemRender>::RENDER;

pub struct ItemDiamondBlockMod;

impl ItemDiamondBlockMod {
    pub fn init(_block: &mut block_diamond_block::BlockDiamondBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
