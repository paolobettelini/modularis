use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BlackConcreteBlockItem;

impl Item for BlackConcreteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:black_concrete_block",
        label: "Black Concrete",
    };
}

impl ItemRender for BlackConcreteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-black-concrete-block:item/black_concrete_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BlackConcreteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BlackConcreteBlockItem as ItemRender>::RENDER;

pub struct ItemBlackConcreteBlockMod;

impl ItemBlackConcreteBlockMod {
    pub fn init(_block: &mut block_black_concrete::BlockBlackConcreteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
