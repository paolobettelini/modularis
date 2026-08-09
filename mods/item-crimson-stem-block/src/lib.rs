use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CrimsonStemBlockItem;

impl Item for CrimsonStemBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:crimson_stem_block",
        label: "Crimson Stem",
    };
}

impl ItemRender for CrimsonStemBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-crimson-stem-block:item/crimson_stem_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CrimsonStemBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CrimsonStemBlockItem as ItemRender>::RENDER;

pub struct ItemCrimsonStemBlockMod;

impl ItemCrimsonStemBlockMod {
    pub fn init(_block: &mut block_crimson_stem::BlockCrimsonStemMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
