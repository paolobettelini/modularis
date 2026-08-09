use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MushroomBlockInsideBlockItem;

impl Item for MushroomBlockInsideBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:mushroom_block_inside_block",
        label: "Mushroom Block Inside",
    };
}

impl ItemRender for MushroomBlockInsideBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-mushroom-block-inside-block:item/mushroom_block_inside_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MushroomBlockInsideBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MushroomBlockInsideBlockItem as ItemRender>::RENDER;

pub struct ItemMushroomBlockInsideBlockMod;

impl ItemMushroomBlockInsideBlockMod {
    pub fn init(_block: &mut block_mushroom_block_inside::BlockMushroomBlockInsideMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
