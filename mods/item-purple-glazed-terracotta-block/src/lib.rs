use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PurpleGlazedTerracottaBlockItem;

impl Item for PurpleGlazedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:purple_glazed_terracotta_block",
        label: "Purple Glazed Terracotta",
    };
}

impl ItemRender for PurpleGlazedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-purple-glazed-terracotta-block:item/purple_glazed_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PurpleGlazedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <PurpleGlazedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemPurpleGlazedTerracottaBlockMod;

impl ItemPurpleGlazedTerracottaBlockMod {
    pub fn init(
        _block: &mut block_purple_glazed_terracotta::BlockPurpleGlazedTerracottaMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
