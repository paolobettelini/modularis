use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GrayConcretePowderBlockItem;

impl Item for GrayConcretePowderBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:gray_concrete_powder_block",
        label: "Gray Concrete Powder",
    };
}

impl ItemRender for GrayConcretePowderBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-gray-concrete-powder-block:item/gray_concrete_powder_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GrayConcretePowderBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GrayConcretePowderBlockItem as ItemRender>::RENDER;

pub struct ItemGrayConcretePowderBlockMod;

impl ItemGrayConcretePowderBlockMod {
    pub fn init(_block: &mut block_gray_concrete_powder::BlockGrayConcretePowderMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
