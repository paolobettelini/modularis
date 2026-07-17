use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct SnowBlockItem;
impl Item for SnowBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:snow_block",
        label: "Snow",
    };
}

impl ItemRender for SnowBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-snow-block:item/snow_block"),
    };
}
pub const ITEM_INFO: ItemInfo = SnowBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SnowBlockItem as ItemRender>::RENDER;
pub struct ItemSnowBlockMod;
impl ItemSnowBlockMod {
    pub fn init(_block: &mut block_snow::BlockSnowMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
