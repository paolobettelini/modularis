use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct LightBlueConcreteBlockItem;

impl Item for LightBlueConcreteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:light_blue_concrete_block",
        label: "Light Blue Concrete",
    };
}

impl ItemRender for LightBlueConcreteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-light-blue-concrete-block:item/light_blue_concrete_block"),
    };
}

pub const ITEM_INFO: ItemInfo = LightBlueConcreteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <LightBlueConcreteBlockItem as ItemRender>::RENDER;

pub struct ItemLightBlueConcreteBlockMod;

impl ItemLightBlueConcreteBlockMod {
    pub fn init(_block: &mut block_light_blue_concrete::BlockLightBlueConcreteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
