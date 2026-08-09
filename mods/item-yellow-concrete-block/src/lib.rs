use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct YellowConcreteBlockItem;

impl Item for YellowConcreteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:yellow_concrete_block",
        label: "Yellow Concrete",
    };
}

impl ItemRender for YellowConcreteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-yellow-concrete-block:item/yellow_concrete_block"),
    };
}

pub const ITEM_INFO: ItemInfo = YellowConcreteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <YellowConcreteBlockItem as ItemRender>::RENDER;

pub struct ItemYellowConcreteBlockMod;

impl ItemYellowConcreteBlockMod {
    pub fn init(_block: &mut block_yellow_concrete::BlockYellowConcreteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
