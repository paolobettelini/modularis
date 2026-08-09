use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BrownTerracottaBlockItem;

impl Item for BrownTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:brown_terracotta_block",
        label: "Brown Terracotta",
    };
}

impl ItemRender for BrownTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-brown-terracotta-block:item/brown_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BrownTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BrownTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemBrownTerracottaBlockMod;

impl ItemBrownTerracottaBlockMod {
    pub fn init(_block: &mut block_brown_terracotta::BlockBrownTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
