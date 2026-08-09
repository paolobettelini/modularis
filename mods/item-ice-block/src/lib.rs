use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct IceBlockItem;

impl Item for IceBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:ice_block",
        label: "Ice",
    };
}

impl ItemRender for IceBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-ice-block:item/ice_block"),
    };
}

pub const ITEM_INFO: ItemInfo = IceBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <IceBlockItem as ItemRender>::RENDER;

pub struct ItemIceBlockMod;

impl ItemIceBlockMod {
    pub fn init(_block: &mut block_ice::BlockIceMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
