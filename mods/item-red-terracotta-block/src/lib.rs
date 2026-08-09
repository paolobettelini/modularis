use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct RedTerracottaBlockItem;

impl Item for RedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:red_terracotta_block",
        label: "Red Terracotta",
    };
}

impl ItemRender for RedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-red-terracotta-block:item/red_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = RedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <RedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemRedTerracottaBlockMod;

impl ItemRedTerracottaBlockMod {
    pub fn init(_block: &mut block_red_terracotta::BlockRedTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
