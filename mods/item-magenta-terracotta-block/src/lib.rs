use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MagentaTerracottaBlockItem;

impl Item for MagentaTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:magenta_terracotta_block",
        label: "Magenta Terracotta",
    };
}

impl ItemRender for MagentaTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-magenta-terracotta-block:item/magenta_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MagentaTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MagentaTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemMagentaTerracottaBlockMod;

impl ItemMagentaTerracottaBlockMod {
    pub fn init(_block: &mut block_magenta_terracotta::BlockMagentaTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
