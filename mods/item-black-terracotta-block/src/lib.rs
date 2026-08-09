use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BlackTerracottaBlockItem;

impl Item for BlackTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:black_terracotta_block",
        label: "Black Terracotta",
    };
}

impl ItemRender for BlackTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-black-terracotta-block:item/black_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BlackTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BlackTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemBlackTerracottaBlockMod;

impl ItemBlackTerracottaBlockMod {
    pub fn init(_block: &mut block_black_terracotta::BlockBlackTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
