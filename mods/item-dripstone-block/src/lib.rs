use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DripstoneBlockItem;

impl Item for DripstoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:dripstone_block",
        label: "Dripstone Block",
    };
}

impl ItemRender for DripstoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-dripstone-block:item/dripstone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DripstoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DripstoneBlockItem as ItemRender>::RENDER;

pub struct ItemDripstoneBlockMod;

impl ItemDripstoneBlockMod {
    pub fn init(_block: &mut block_dripstone_block::BlockDripstoneBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
