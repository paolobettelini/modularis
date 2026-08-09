use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PinkTerracottaBlockItem;

impl Item for PinkTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:pink_terracotta_block",
        label: "Pink Terracotta",
    };
}

impl ItemRender for PinkTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-pink-terracotta-block:item/pink_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PinkTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PinkTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemPinkTerracottaBlockMod;

impl ItemPinkTerracottaBlockMod {
    pub fn init(_block: &mut block_pink_terracotta::BlockPinkTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
