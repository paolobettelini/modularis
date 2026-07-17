use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct CrimsonNyliumBlockItem;
impl Item for CrimsonNyliumBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:crimson_nylium_block",
        label: "Crimson Nylium",
    };
}

impl ItemRender for CrimsonNyliumBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-crimson-nylium-block:item/crimson_nylium_block"),
    };
}
pub const ITEM_INFO: ItemInfo = CrimsonNyliumBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CrimsonNyliumBlockItem as ItemRender>::RENDER;
pub struct ItemCrimsonNyliumBlockMod;
impl ItemCrimsonNyliumBlockMod {
    pub fn init(_block: &mut block_crimson_nylium::BlockCrimsonNyliumMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
