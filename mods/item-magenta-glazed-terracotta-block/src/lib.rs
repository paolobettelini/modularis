use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MagentaGlazedTerracottaBlockItem;

impl Item for MagentaGlazedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:magenta_glazed_terracotta_block",
        label: "Magenta Glazed Terracotta",
    };
}

impl ItemRender for MagentaGlazedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-magenta-glazed-terracotta-block:item/magenta_glazed_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MagentaGlazedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <MagentaGlazedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemMagentaGlazedTerracottaBlockMod;

impl ItemMagentaGlazedTerracottaBlockMod {
    pub fn init(
        _block: &mut block_magenta_glazed_terracotta::BlockMagentaGlazedTerracottaMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
