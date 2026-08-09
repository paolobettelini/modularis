use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BlueGlazedTerracottaBlockItem;

impl Item for BlueGlazedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:blue_glazed_terracotta_block",
        label: "Blue Glazed Terracotta",
    };
}

impl ItemRender for BlueGlazedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-blue-glazed-terracotta-block:item/blue_glazed_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BlueGlazedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BlueGlazedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemBlueGlazedTerracottaBlockMod;

impl ItemBlueGlazedTerracottaBlockMod {
    pub fn init(_block: &mut block_blue_glazed_terracotta::BlockBlueGlazedTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
