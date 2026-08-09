use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PinkConcretePowderBlockItem;

impl Item for PinkConcretePowderBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:pink_concrete_powder_block",
        label: "Pink Concrete Powder",
    };
}

impl ItemRender for PinkConcretePowderBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-pink-concrete-powder-block:item/pink_concrete_powder_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PinkConcretePowderBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PinkConcretePowderBlockItem as ItemRender>::RENDER;

pub struct ItemPinkConcretePowderBlockMod;

impl ItemPinkConcretePowderBlockMod {
    pub fn init(_block: &mut block_pink_concrete_powder::BlockPinkConcretePowderMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
