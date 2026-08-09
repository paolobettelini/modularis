use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GrayConcreteBlockItem;

impl Item for GrayConcreteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:gray_concrete_block",
        label: "Gray Concrete",
    };
}

impl ItemRender for GrayConcreteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-gray-concrete-block:item/gray_concrete_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GrayConcreteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GrayConcreteBlockItem as ItemRender>::RENDER;

pub struct ItemGrayConcreteBlockMod;

impl ItemGrayConcreteBlockMod {
    pub fn init(_block: &mut block_gray_concrete::BlockGrayConcreteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
