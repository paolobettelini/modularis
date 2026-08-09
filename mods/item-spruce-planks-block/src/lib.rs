use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct SprucePlanksBlockItem;

impl Item for SprucePlanksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:spruce_planks_block",
        label: "Spruce Planks",
    };
}

impl ItemRender for SprucePlanksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-spruce-planks-block:item/spruce_planks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = SprucePlanksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SprucePlanksBlockItem as ItemRender>::RENDER;

pub struct ItemSprucePlanksBlockMod;

impl ItemSprucePlanksBlockMod {
    pub fn init(_block: &mut block_spruce_planks::BlockSprucePlanksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
