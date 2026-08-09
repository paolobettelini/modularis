use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PurpurBlockItem;

impl Item for PurpurBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:purpur_block",
        label: "Purpur Block",
    };
}

impl ItemRender for PurpurBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-purpur-block:item/purpur_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PurpurBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PurpurBlockItem as ItemRender>::RENDER;

pub struct ItemPurpurBlockMod;

impl ItemPurpurBlockMod {
    pub fn init(_block: &mut block_purpur_block::BlockPurpurBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
