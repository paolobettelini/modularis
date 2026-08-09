use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct SeaLanternBlockItem;

impl Item for SeaLanternBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:sea_lantern_block",
        label: "Sea Lantern",
    };
}

impl ItemRender for SeaLanternBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-sea-lantern-block:item/sea_lantern_block"),
    };
}

pub const ITEM_INFO: ItemInfo = SeaLanternBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SeaLanternBlockItem as ItemRender>::RENDER;

pub struct ItemSeaLanternBlockMod;

impl ItemSeaLanternBlockMod {
    pub fn init(_block: &mut block_sea_lantern::BlockSeaLanternMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
