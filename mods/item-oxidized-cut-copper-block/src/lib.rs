use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct OxidizedCutCopperBlockItem;

impl Item for OxidizedCutCopperBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:oxidized_cut_copper_block",
        label: "Oxidized Cut Copper",
    };
}

impl ItemRender for OxidizedCutCopperBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-oxidized-cut-copper-block:item/oxidized_cut_copper_block"),
    };
}

pub const ITEM_INFO: ItemInfo = OxidizedCutCopperBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <OxidizedCutCopperBlockItem as ItemRender>::RENDER;

pub struct ItemOxidizedCutCopperBlockMod;

impl ItemOxidizedCutCopperBlockMod {
    pub fn init(_block: &mut block_oxidized_cut_copper::BlockOxidizedCutCopperMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
