use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LightGrayConcreteBlockItem;

impl Item for LightGrayConcreteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:light_gray_concrete_block",
        label: "Light Gray Concrete",
    };
}

impl ItemRender for LightGrayConcreteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-light-gray-concrete-block:item/light_gray_concrete_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LightGrayConcreteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <LightGrayConcreteBlockItem as ItemRender>::RENDER;

pub struct ItemLightGrayConcreteBlockMod;

impl ItemLightGrayConcreteBlockMod {
    pub fn init(_block: &mut block_light_gray_concrete::BlockLightGrayConcreteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
