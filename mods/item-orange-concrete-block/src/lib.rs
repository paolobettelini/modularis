use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct OrangeConcreteBlockItem;

impl Item for OrangeConcreteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:orange_concrete_block",
        label: "Orange Concrete",
    };
}

impl ItemRender for OrangeConcreteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-orange-concrete-block:item/orange_concrete_block"),
    };
}

pub const ITEM_INFO: ItemInfo = OrangeConcreteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <OrangeConcreteBlockItem as ItemRender>::RENDER;

pub struct ItemOrangeConcreteBlockMod;

impl ItemOrangeConcreteBlockMod {
    pub fn init(_block: &mut block_orange_concrete::BlockOrangeConcreteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
