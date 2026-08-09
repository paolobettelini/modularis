use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct StrippedBambooBlockItem;

impl Item for StrippedBambooBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:stripped_bamboo_block",
        label: "Stripped Bamboo Block",
    };
}

impl ItemRender for StrippedBambooBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-stripped-bamboo-block:item/stripped_bamboo_block"),
    };
}

pub const ITEM_INFO: ItemInfo = StrippedBambooBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <StrippedBambooBlockItem as ItemRender>::RENDER;

pub struct ItemStrippedBambooBlockMod;

impl ItemStrippedBambooBlockMod {
    pub fn init(_block: &mut block_stripped_bamboo_block::BlockStrippedBambooBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
