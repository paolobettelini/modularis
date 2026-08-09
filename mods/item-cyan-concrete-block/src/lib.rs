use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CyanConcreteBlockItem;

impl Item for CyanConcreteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cyan_concrete_block",
        label: "Cyan Concrete",
    };
}

impl ItemRender for CyanConcreteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cyan-concrete-block:item/cyan_concrete_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CyanConcreteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CyanConcreteBlockItem as ItemRender>::RENDER;

pub struct ItemCyanConcreteBlockMod;

impl ItemCyanConcreteBlockMod {
    pub fn init(_block: &mut block_cyan_concrete::BlockCyanConcreteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
