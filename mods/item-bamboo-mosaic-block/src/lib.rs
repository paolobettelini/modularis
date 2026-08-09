use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BambooMosaicBlockItem;

impl Item for BambooMosaicBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:bamboo_mosaic_block",
        label: "Bamboo Mosaic",
    };
}

impl ItemRender for BambooMosaicBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-bamboo-mosaic-block:item/bamboo_mosaic_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BambooMosaicBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BambooMosaicBlockItem as ItemRender>::RENDER;

pub struct ItemBambooMosaicBlockMod;

impl ItemBambooMosaicBlockMod {
    pub fn init(_block: &mut block_bamboo_mosaic::BlockBambooMosaicMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
