use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LightGrayConcretePowderBlockItem;

impl Item for LightGrayConcretePowderBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:light_gray_concrete_powder_block",
        label: "Light Gray Concrete Powder",
    };
}

impl ItemRender for LightGrayConcretePowderBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-light-gray-concrete-powder-block:item/light_gray_concrete_powder_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LightGrayConcretePowderBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <LightGrayConcretePowderBlockItem as ItemRender>::RENDER;

pub struct ItemLightGrayConcretePowderBlockMod;

impl ItemLightGrayConcretePowderBlockMod {
    pub fn init(
        _block: &mut block_light_gray_concrete_powder::BlockLightGrayConcretePowderMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
