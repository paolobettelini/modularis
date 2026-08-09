use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BlueIceBlockItem;

impl Item for BlueIceBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:blue_ice_block",
        label: "Blue Ice",
    };
}

impl ItemRender for BlueIceBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-blue-ice-block:item/blue_ice_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BlueIceBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BlueIceBlockItem as ItemRender>::RENDER;

pub struct ItemBlueIceBlockMod;

impl ItemBlueIceBlockMod {
    pub fn init(_block: &mut block_blue_ice::BlockBlueIceMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
