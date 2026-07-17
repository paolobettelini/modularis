use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct CactusBlockItem;
impl Item for CactusBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cactus_block",
        label: "Cactus",
    };
}

impl ItemRender for CactusBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cactus-block:item/cactus_block"),
    };
}
pub const ITEM_INFO: ItemInfo = CactusBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CactusBlockItem as ItemRender>::RENDER;
pub struct ItemCactusBlockMod;
impl ItemCactusBlockMod {
    pub fn init(_block: &mut block_cactus::BlockCactusMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
