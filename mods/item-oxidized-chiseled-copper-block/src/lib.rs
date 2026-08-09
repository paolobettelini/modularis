use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct OxidizedChiseledCopperBlockItem;

impl Item for OxidizedChiseledCopperBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:oxidized_chiseled_copper_block",
        label: "Oxidized Chiseled Copper",
    };
}

impl ItemRender for OxidizedChiseledCopperBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-oxidized-chiseled-copper-block:item/oxidized_chiseled_copper_block"),
    };
}

pub const ITEM_INFO: ItemInfo = OxidizedChiseledCopperBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <OxidizedChiseledCopperBlockItem as ItemRender>::RENDER;

pub struct ItemOxidizedChiseledCopperBlockMod;

impl ItemOxidizedChiseledCopperBlockMod {
    pub fn init(
        _block: &mut block_oxidized_chiseled_copper::BlockOxidizedChiseledCopperMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
