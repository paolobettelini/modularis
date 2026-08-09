use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct OrangeTerracottaBlockItem;

impl Item for OrangeTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:orange_terracotta_block",
        label: "Orange Terracotta",
    };
}

impl ItemRender for OrangeTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-orange-terracotta-block:item/orange_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = OrangeTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <OrangeTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemOrangeTerracottaBlockMod;

impl ItemOrangeTerracottaBlockMod {
    pub fn init(_block: &mut block_orange_terracotta::BlockOrangeTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
