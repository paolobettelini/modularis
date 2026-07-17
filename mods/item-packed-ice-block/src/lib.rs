use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct PackedIceBlockItem;
impl Item for PackedIceBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:packed_ice_block",
        label: "Packed Ice",
    };
}

impl ItemRender for PackedIceBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-packed-ice-block:item/packed_ice_block"),
    };
}
pub const ITEM_INFO: ItemInfo = PackedIceBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PackedIceBlockItem as ItemRender>::RENDER;
pub struct ItemPackedIceBlockMod;
impl ItemPackedIceBlockMod {
    pub fn init(_block: &mut block_packed_ice::BlockPackedIceMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
