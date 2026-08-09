use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct OxidizedCopperBlockItem;

impl Item for OxidizedCopperBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:oxidized_copper_block",
        label: "Oxidized Copper",
    };
}

impl ItemRender for OxidizedCopperBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-oxidized-copper-block:item/oxidized_copper_block"),
    };
}

pub const ITEM_INFO: ItemInfo = OxidizedCopperBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <OxidizedCopperBlockItem as ItemRender>::RENDER;

pub struct ItemOxidizedCopperBlockMod;

impl ItemOxidizedCopperBlockMod {
    pub fn init(_block: &mut block_oxidized_copper::BlockOxidizedCopperMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
