use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GreenTerracottaBlockItem;

impl Item for GreenTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:green_terracotta_block",
        label: "Green Terracotta",
    };
}

impl ItemRender for GreenTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-green-terracotta-block:item/green_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GreenTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GreenTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemGreenTerracottaBlockMod;

impl ItemGreenTerracottaBlockMod {
    pub fn init(_block: &mut block_green_terracotta::BlockGreenTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
