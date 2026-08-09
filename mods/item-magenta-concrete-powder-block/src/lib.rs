use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MagentaConcretePowderBlockItem;

impl Item for MagentaConcretePowderBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:magenta_concrete_powder_block",
        label: "Magenta Concrete Powder",
    };
}

impl ItemRender for MagentaConcretePowderBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-magenta-concrete-powder-block:item/magenta_concrete_powder_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MagentaConcretePowderBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MagentaConcretePowderBlockItem as ItemRender>::RENDER;

pub struct ItemMagentaConcretePowderBlockMod;

impl ItemMagentaConcretePowderBlockMod {
    pub fn init(_block: &mut block_magenta_concrete_powder::BlockMagentaConcretePowderMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
