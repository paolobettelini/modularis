use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BlueConcreteBlockItem;

impl Item for BlueConcreteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:blue_concrete_block",
        label: "Blue Concrete",
    };
}

impl ItemRender for BlueConcreteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-blue-concrete-block:item/blue_concrete_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BlueConcreteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BlueConcreteBlockItem as ItemRender>::RENDER;

pub struct ItemBlueConcreteBlockMod;

impl ItemBlueConcreteBlockMod {
    pub fn init(_block: &mut block_blue_concrete::BlockBlueConcreteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
