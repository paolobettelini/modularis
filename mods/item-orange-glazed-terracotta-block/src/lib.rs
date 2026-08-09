use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct OrangeGlazedTerracottaBlockItem;

impl Item for OrangeGlazedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:orange_glazed_terracotta_block",
        label: "Orange Glazed Terracotta",
    };
}

impl ItemRender for OrangeGlazedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-orange-glazed-terracotta-block:item/orange_glazed_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = OrangeGlazedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <OrangeGlazedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemOrangeGlazedTerracottaBlockMod;

impl ItemOrangeGlazedTerracottaBlockMod {
    pub fn init(
        _block: &mut block_orange_glazed_terracotta::BlockOrangeGlazedTerracottaMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
