use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BrownConcretePowderBlockItem;

impl Item for BrownConcretePowderBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:brown_concrete_powder_block",
        label: "Brown Concrete Powder",
    };
}

impl ItemRender for BrownConcretePowderBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-brown-concrete-powder-block:item/brown_concrete_powder_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BrownConcretePowderBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BrownConcretePowderBlockItem as ItemRender>::RENDER;

pub struct ItemBrownConcretePowderBlockMod;

impl ItemBrownConcretePowderBlockMod {
    pub fn init(_block: &mut block_brown_concrete_powder::BlockBrownConcretePowderMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
