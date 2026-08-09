use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct RedConcretePowderBlockItem;

impl Item for RedConcretePowderBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:red_concrete_powder_block",
        label: "Red Concrete Powder",
    };
}

impl ItemRender for RedConcretePowderBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-red-concrete-powder-block:item/red_concrete_powder_block"),
    };
}

pub const ITEM_INFO: ItemInfo = RedConcretePowderBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <RedConcretePowderBlockItem as ItemRender>::RENDER;

pub struct ItemRedConcretePowderBlockMod;

impl ItemRedConcretePowderBlockMod {
    pub fn init(_block: &mut block_red_concrete_powder::BlockRedConcretePowderMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
