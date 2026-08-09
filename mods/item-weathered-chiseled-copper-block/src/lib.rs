use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct WeatheredChiseledCopperBlockItem;

impl Item for WeatheredChiseledCopperBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:weathered_chiseled_copper_block",
        label: "Weathered Chiseled Copper",
    };
}

impl ItemRender for WeatheredChiseledCopperBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-weathered-chiseled-copper-block:item/weathered_chiseled_copper_block"),
    };
}

pub const ITEM_INFO: ItemInfo = WeatheredChiseledCopperBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <WeatheredChiseledCopperBlockItem as ItemRender>::RENDER;

pub struct ItemWeatheredChiseledCopperBlockMod;

impl ItemWeatheredChiseledCopperBlockMod {
    pub fn init(
        _block: &mut block_weathered_chiseled_copper::BlockWeatheredChiseledCopperMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
