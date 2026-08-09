use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MagentaConcreteBlockItem;

impl Item for MagentaConcreteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:magenta_concrete_block",
        label: "Magenta Concrete",
    };
}

impl ItemRender for MagentaConcreteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-magenta-concrete-block:item/magenta_concrete_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MagentaConcreteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MagentaConcreteBlockItem as ItemRender>::RENDER;

pub struct ItemMagentaConcreteBlockMod;

impl ItemMagentaConcreteBlockMod {
    pub fn init(_block: &mut block_magenta_concrete::BlockMagentaConcreteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
