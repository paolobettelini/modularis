use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct YellowConcretePowderBlockItem;

impl Item for YellowConcretePowderBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:yellow_concrete_powder_block",
        label: "Yellow Concrete Powder",
    };
}

impl ItemRender for YellowConcretePowderBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-yellow-concrete-powder-block:item/yellow_concrete_powder_block"),
    };
}

pub const ITEM_INFO: ItemInfo = YellowConcretePowderBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <YellowConcretePowderBlockItem as ItemRender>::RENDER;

pub struct ItemYellowConcretePowderBlockMod;

impl ItemYellowConcretePowderBlockMod {
    pub fn init(_block: &mut block_yellow_concrete_powder::BlockYellowConcretePowderMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
