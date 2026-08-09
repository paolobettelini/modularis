use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GreenConcreteBlockItem;

impl Item for GreenConcreteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:green_concrete_block",
        label: "Green Concrete",
    };
}

impl ItemRender for GreenConcreteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-green-concrete-block:item/green_concrete_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GreenConcreteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GreenConcreteBlockItem as ItemRender>::RENDER;

pub struct ItemGreenConcreteBlockMod;

impl ItemGreenConcreteBlockMod {
    pub fn init(_block: &mut block_green_concrete::BlockGreenConcreteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
