use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LightBlueConcretePowderBlockItem;

impl Item for LightBlueConcretePowderBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:light_blue_concrete_powder_block",
        label: "Light Blue Concrete Powder",
    };
}

impl ItemRender for LightBlueConcretePowderBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-light-blue-concrete-powder-block:item/light_blue_concrete_powder_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LightBlueConcretePowderBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <LightBlueConcretePowderBlockItem as ItemRender>::RENDER;

pub struct ItemLightBlueConcretePowderBlockMod;

impl ItemLightBlueConcretePowderBlockMod {
    pub fn init(
        _block: &mut block_light_blue_concrete_powder::BlockLightBlueConcretePowderMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
