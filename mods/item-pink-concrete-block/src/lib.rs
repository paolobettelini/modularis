use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PinkConcreteBlockItem;

impl Item for PinkConcreteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:pink_concrete_block",
        label: "Pink Concrete",
    };
}

impl ItemRender for PinkConcreteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-pink-concrete-block:item/pink_concrete_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PinkConcreteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PinkConcreteBlockItem as ItemRender>::RENDER;

pub struct ItemPinkConcreteBlockMod;

impl ItemPinkConcreteBlockMod {
    pub fn init(_block: &mut block_pink_concrete::BlockPinkConcreteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
