use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ExposedChiseledCopperBlockItem;

impl Item for ExposedChiseledCopperBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:exposed_chiseled_copper_block",
        label: "Exposed Chiseled Copper",
    };
}

impl ItemRender for ExposedChiseledCopperBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-exposed-chiseled-copper-block:item/exposed_chiseled_copper_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ExposedChiseledCopperBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ExposedChiseledCopperBlockItem as ItemRender>::RENDER;

pub struct ItemExposedChiseledCopperBlockMod;

impl ItemExposedChiseledCopperBlockMod {
    pub fn init(_block: &mut block_exposed_chiseled_copper::BlockExposedChiseledCopperMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
