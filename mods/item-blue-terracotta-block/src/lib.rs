use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BlueTerracottaBlockItem;

impl Item for BlueTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:blue_terracotta_block",
        label: "Blue Terracotta",
    };
}

impl ItemRender for BlueTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-blue-terracotta-block:item/blue_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BlueTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BlueTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemBlueTerracottaBlockMod;

impl ItemBlueTerracottaBlockMod {
    pub fn init(_block: &mut block_blue_terracotta::BlockBlueTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
