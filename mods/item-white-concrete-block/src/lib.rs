use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct WhiteConcreteBlockItem;

impl Item for WhiteConcreteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:white_concrete_block",
        label: "White Concrete",
    };
}

impl ItemRender for WhiteConcreteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-white-concrete-block:item/white_concrete_block"),
    };
}

pub const ITEM_INFO: ItemInfo = WhiteConcreteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <WhiteConcreteBlockItem as ItemRender>::RENDER;

pub struct ItemWhiteConcreteBlockMod;

impl ItemWhiteConcreteBlockMod {
    pub fn init(_block: &mut block_white_concrete::BlockWhiteConcreteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
