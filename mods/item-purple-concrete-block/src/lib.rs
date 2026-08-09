use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PurpleConcreteBlockItem;

impl Item for PurpleConcreteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:purple_concrete_block",
        label: "Purple Concrete",
    };
}

impl ItemRender for PurpleConcreteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-purple-concrete-block:item/purple_concrete_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PurpleConcreteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PurpleConcreteBlockItem as ItemRender>::RENDER;

pub struct ItemPurpleConcreteBlockMod;

impl ItemPurpleConcreteBlockMod {
    pub fn init(_block: &mut block_purple_concrete::BlockPurpleConcreteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
