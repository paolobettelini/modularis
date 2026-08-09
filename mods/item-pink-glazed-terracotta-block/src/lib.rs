use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PinkGlazedTerracottaBlockItem;

impl Item for PinkGlazedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:pink_glazed_terracotta_block",
        label: "Pink Glazed Terracotta",
    };
}

impl ItemRender for PinkGlazedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-pink-glazed-terracotta-block:item/pink_glazed_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PinkGlazedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PinkGlazedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemPinkGlazedTerracottaBlockMod;

impl ItemPinkGlazedTerracottaBlockMod {
    pub fn init(_block: &mut block_pink_glazed_terracotta::BlockPinkGlazedTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
