use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PurpleTerracottaBlockItem;

impl Item for PurpleTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:purple_terracotta_block",
        label: "Purple Terracotta",
    };
}

impl ItemRender for PurpleTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-purple-terracotta-block:item/purple_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PurpleTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PurpleTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemPurpleTerracottaBlockMod;

impl ItemPurpleTerracottaBlockMod {
    pub fn init(_block: &mut block_purple_terracotta::BlockPurpleTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
