use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct OrangeConcretePowderBlockItem;

impl Item for OrangeConcretePowderBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:orange_concrete_powder_block",
        label: "Orange Concrete Powder",
    };
}

impl ItemRender for OrangeConcretePowderBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-orange-concrete-powder-block:item/orange_concrete_powder_block"),
    };
}

pub const ITEM_INFO: ItemInfo = OrangeConcretePowderBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <OrangeConcretePowderBlockItem as ItemRender>::RENDER;

pub struct ItemOrangeConcretePowderBlockMod;

impl ItemOrangeConcretePowderBlockMod {
    pub fn init(_block: &mut block_orange_concrete_powder::BlockOrangeConcretePowderMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
