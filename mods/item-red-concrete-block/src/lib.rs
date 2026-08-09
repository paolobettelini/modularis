use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct RedConcreteBlockItem;

impl Item for RedConcreteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:red_concrete_block",
        label: "Red Concrete",
    };
}

impl ItemRender for RedConcreteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-red-concrete-block:item/red_concrete_block"),
    };
}

pub const ITEM_INFO: ItemInfo = RedConcreteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <RedConcreteBlockItem as ItemRender>::RENDER;

pub struct ItemRedConcreteBlockMod;

impl ItemRedConcreteBlockMod {
    pub fn init(_block: &mut block_red_concrete::BlockRedConcreteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
