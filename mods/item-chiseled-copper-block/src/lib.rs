use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ChiseledCopperBlockItem;

impl Item for ChiseledCopperBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:chiseled_copper_block",
        label: "Chiseled Copper",
    };
}

impl ItemRender for ChiseledCopperBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-chiseled-copper-block:item/chiseled_copper_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ChiseledCopperBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ChiseledCopperBlockItem as ItemRender>::RENDER;

pub struct ItemChiseledCopperBlockMod;

impl ItemChiseledCopperBlockMod {
    pub fn init(_block: &mut block_chiseled_copper::BlockChiseledCopperMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
