use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LimeConcretePowderBlockItem;

impl Item for LimeConcretePowderBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:lime_concrete_powder_block",
        label: "Lime Concrete Powder",
    };
}

impl ItemRender for LimeConcretePowderBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-lime-concrete-powder-block:item/lime_concrete_powder_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LimeConcretePowderBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <LimeConcretePowderBlockItem as ItemRender>::RENDER;

pub struct ItemLimeConcretePowderBlockMod;

impl ItemLimeConcretePowderBlockMod {
    pub fn init(_block: &mut block_lime_concrete_powder::BlockLimeConcretePowderMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
