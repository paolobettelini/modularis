use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct BirchLogBlockItem;
impl Item for BirchLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:birch_log_block",
        label: "Birch Log",
    };
}

impl ItemRender for BirchLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-birch-log-block:item/birch_log_block"),
    };
}
pub const ITEM_INFO: ItemInfo = BirchLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BirchLogBlockItem as ItemRender>::RENDER;
pub struct ItemBirchLogBlockMod;
impl ItemBirchLogBlockMod {
    pub fn init(_block: &mut block_birch_log::BlockBirchLogMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
