use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CyanConcretePowderBlockItem;

impl Item for CyanConcretePowderBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cyan_concrete_powder_block",
        label: "Cyan Concrete Powder",
    };
}

impl ItemRender for CyanConcretePowderBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cyan-concrete-powder-block:item/cyan_concrete_powder_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CyanConcretePowderBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CyanConcretePowderBlockItem as ItemRender>::RENDER;

pub struct ItemCyanConcretePowderBlockMod;

impl ItemCyanConcretePowderBlockMod {
    pub fn init(_block: &mut block_cyan_concrete_powder::BlockCyanConcretePowderMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
