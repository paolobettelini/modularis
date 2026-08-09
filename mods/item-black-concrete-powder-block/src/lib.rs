use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BlackConcretePowderBlockItem;

impl Item for BlackConcretePowderBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:black_concrete_powder_block",
        label: "Black Concrete Powder",
    };
}

impl ItemRender for BlackConcretePowderBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-black-concrete-powder-block:item/black_concrete_powder_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BlackConcretePowderBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BlackConcretePowderBlockItem as ItemRender>::RENDER;

pub struct ItemBlackConcretePowderBlockMod;

impl ItemBlackConcretePowderBlockMod {
    pub fn init(_block: &mut block_black_concrete_powder::BlockBlackConcretePowderMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
