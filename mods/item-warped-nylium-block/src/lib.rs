use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct WarpedNyliumBlockItem;
impl Item for WarpedNyliumBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:warped_nylium_block",
        label: "Warped Nylium",
    };
}

impl ItemRender for WarpedNyliumBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-warped-nylium-block:item/warped_nylium_block"),
    };
}
pub const ITEM_INFO: ItemInfo = WarpedNyliumBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <WarpedNyliumBlockItem as ItemRender>::RENDER;
pub struct ItemWarpedNyliumBlockMod;
impl ItemWarpedNyliumBlockMod {
    pub fn init(_block: &mut block_warped_nylium::BlockWarpedNyliumMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
