use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BambooBlockItem;

impl Item for BambooBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:bamboo_block",
        label: "Bamboo Block",
    };
}

impl ItemRender for BambooBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-bamboo-block:item/bamboo_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BambooBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BambooBlockItem as ItemRender>::RENDER;

pub struct ItemBambooBlockMod;

impl ItemBambooBlockMod {
    pub fn init(_block: &mut block_bamboo_block::BlockBambooBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
