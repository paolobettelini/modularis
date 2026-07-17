use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct SoulSoilBlockItem;
impl Item for SoulSoilBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:soul_soil_block",
        label: "Soul Soil",
    };
}

impl ItemRender for SoulSoilBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-soul-soil-block:item/soul_soil_block"),
    };
}
pub const ITEM_INFO: ItemInfo = SoulSoilBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SoulSoilBlockItem as ItemRender>::RENDER;
pub struct ItemSoulSoilBlockMod;
impl ItemSoulSoilBlockMod {
    pub fn init(_block: &mut block_soul_soil::BlockSoulSoilMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
