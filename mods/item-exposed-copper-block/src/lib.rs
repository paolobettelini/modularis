use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ExposedCopperBlockItem;

impl Item for ExposedCopperBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:exposed_copper_block",
        label: "Exposed Copper",
    };
}

impl ItemRender for ExposedCopperBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-exposed-copper-block:item/exposed_copper_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ExposedCopperBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ExposedCopperBlockItem as ItemRender>::RENDER;

pub struct ItemExposedCopperBlockMod;

impl ItemExposedCopperBlockMod {
    pub fn init(_block: &mut block_exposed_copper::BlockExposedCopperMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
