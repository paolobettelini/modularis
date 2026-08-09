use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BrownGlazedTerracottaBlockItem;

impl Item for BrownGlazedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:brown_glazed_terracotta_block",
        label: "Brown Glazed Terracotta",
    };
}

impl ItemRender for BrownGlazedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-brown-glazed-terracotta-block:item/brown_glazed_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BrownGlazedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BrownGlazedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemBrownGlazedTerracottaBlockMod;

impl ItemBrownGlazedTerracottaBlockMod {
    pub fn init(_block: &mut block_brown_glazed_terracotta::BlockBrownGlazedTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
