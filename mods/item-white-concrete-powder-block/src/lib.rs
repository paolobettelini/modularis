use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct WhiteConcretePowderBlockItem;

impl Item for WhiteConcretePowderBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:white_concrete_powder_block",
        label: "White Concrete Powder",
    };
}

impl ItemRender for WhiteConcretePowderBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-white-concrete-powder-block:item/white_concrete_powder_block"),
    };
}

pub const ITEM_INFO: ItemInfo = WhiteConcretePowderBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <WhiteConcretePowderBlockItem as ItemRender>::RENDER;

pub struct ItemWhiteConcretePowderBlockMod;

impl ItemWhiteConcretePowderBlockMod {
    pub fn init(_block: &mut block_white_concrete_powder::BlockWhiteConcretePowderMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
