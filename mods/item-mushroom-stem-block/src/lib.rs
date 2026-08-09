use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MushroomStemBlockItem;

impl Item for MushroomStemBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:mushroom_stem_block",
        label: "Mushroom Stem",
    };
}

impl ItemRender for MushroomStemBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-mushroom-stem-block:item/mushroom_stem_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MushroomStemBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MushroomStemBlockItem as ItemRender>::RENDER;

pub struct ItemMushroomStemBlockMod;

impl ItemMushroomStemBlockMod {
    pub fn init(_block: &mut block_mushroom_stem::BlockMushroomStemMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
