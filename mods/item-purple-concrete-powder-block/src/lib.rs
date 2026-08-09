use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PurpleConcretePowderBlockItem;

impl Item for PurpleConcretePowderBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:purple_concrete_powder_block",
        label: "Purple Concrete Powder",
    };
}

impl ItemRender for PurpleConcretePowderBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-purple-concrete-powder-block:item/purple_concrete_powder_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PurpleConcretePowderBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PurpleConcretePowderBlockItem as ItemRender>::RENDER;

pub struct ItemPurpleConcretePowderBlockMod;

impl ItemPurpleConcretePowderBlockMod {
    pub fn init(_block: &mut block_purple_concrete_powder::BlockPurpleConcretePowderMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
