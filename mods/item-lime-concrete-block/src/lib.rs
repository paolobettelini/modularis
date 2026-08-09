use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LimeConcreteBlockItem;

impl Item for LimeConcreteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:lime_concrete_block",
        label: "Lime Concrete",
    };
}

impl ItemRender for LimeConcreteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-lime-concrete-block:item/lime_concrete_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LimeConcreteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <LimeConcreteBlockItem as ItemRender>::RENDER;

pub struct ItemLimeConcreteBlockMod;

impl ItemLimeConcreteBlockMod {
    pub fn init(_block: &mut block_lime_concrete::BlockLimeConcreteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
