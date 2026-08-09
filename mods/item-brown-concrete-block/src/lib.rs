use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BrownConcreteBlockItem;

impl Item for BrownConcreteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:brown_concrete_block",
        label: "Brown Concrete",
    };
}

impl ItemRender for BrownConcreteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-brown-concrete-block:item/brown_concrete_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BrownConcreteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BrownConcreteBlockItem as ItemRender>::RENDER;

pub struct ItemBrownConcreteBlockMod;

impl ItemBrownConcreteBlockMod {
    pub fn init(_block: &mut block_brown_concrete::BlockBrownConcreteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
