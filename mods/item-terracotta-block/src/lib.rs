use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct TerracottaBlockItem;
impl Item for TerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:terracotta_block",
        label: "Terracotta",
    };
}

impl ItemRender for TerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-terracotta-block:item/terracotta_block"),
    };
}
pub const ITEM_INFO: ItemInfo = TerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <TerracottaBlockItem as ItemRender>::RENDER;
pub struct ItemTerracottaBlockMod;
impl ItemTerracottaBlockMod {
    pub fn init(_block: &mut block_terracotta::BlockTerracottaMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
