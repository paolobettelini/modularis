use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GreenConcretePowderBlockItem;

impl Item for GreenConcretePowderBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:green_concrete_powder_block",
        label: "Green Concrete Powder",
    };
}

impl ItemRender for GreenConcretePowderBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-green-concrete-powder-block:item/green_concrete_powder_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GreenConcretePowderBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GreenConcretePowderBlockItem as ItemRender>::RENDER;

pub struct ItemGreenConcretePowderBlockMod;

impl ItemGreenConcretePowderBlockMod {
    pub fn init(_block: &mut block_green_concrete_powder::BlockGreenConcretePowderMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
