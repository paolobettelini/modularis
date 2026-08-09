use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BlueConcretePowderBlockItem;

impl Item for BlueConcretePowderBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:blue_concrete_powder_block",
        label: "Blue Concrete Powder",
    };
}

impl ItemRender for BlueConcretePowderBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-blue-concrete-powder-block:item/blue_concrete_powder_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BlueConcretePowderBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BlueConcretePowderBlockItem as ItemRender>::RENDER;

pub struct ItemBlueConcretePowderBlockMod;

impl ItemBlueConcretePowderBlockMod {
    pub fn init(_block: &mut block_blue_concrete_powder::BlockBlueConcretePowderMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
