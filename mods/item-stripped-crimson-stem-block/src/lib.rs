use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct StrippedCrimsonStemBlockItem;

impl Item for StrippedCrimsonStemBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:stripped_crimson_stem_block",
        label: "Stripped Crimson Stem",
    };
}

impl ItemRender for StrippedCrimsonStemBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-stripped-crimson-stem-block:item/stripped_crimson_stem_block"),
    };
}

pub const ITEM_INFO: ItemInfo = StrippedCrimsonStemBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <StrippedCrimsonStemBlockItem as ItemRender>::RENDER;

pub struct ItemStrippedCrimsonStemBlockMod;

impl ItemStrippedCrimsonStemBlockMod {
    pub fn init(_block: &mut block_stripped_crimson_stem::BlockStrippedCrimsonStemMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
