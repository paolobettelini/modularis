use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CyanTerracottaBlockItem;

impl Item for CyanTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cyan_terracotta_block",
        label: "Cyan Terracotta",
    };
}

impl ItemRender for CyanTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cyan-terracotta-block:item/cyan_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CyanTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CyanTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemCyanTerracottaBlockMod;

impl ItemCyanTerracottaBlockMod {
    pub fn init(_block: &mut block_cyan_terracotta::BlockCyanTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
