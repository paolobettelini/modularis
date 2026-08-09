use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GreenGlazedTerracottaBlockItem;

impl Item for GreenGlazedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:green_glazed_terracotta_block",
        label: "Green Glazed Terracotta",
    };
}

impl ItemRender for GreenGlazedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-green-glazed-terracotta-block:item/green_glazed_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GreenGlazedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GreenGlazedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemGreenGlazedTerracottaBlockMod;

impl ItemGreenGlazedTerracottaBlockMod {
    pub fn init(_block: &mut block_green_glazed_terracotta::BlockGreenGlazedTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
