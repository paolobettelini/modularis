use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CoalBlockItem;

impl Item for CoalBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:coal_block",
        label: "Coal Block",
    };
}

impl ItemRender for CoalBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-coal-block:item/coal_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CoalBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CoalBlockItem as ItemRender>::RENDER;

pub struct ItemCoalBlockMod;

impl ItemCoalBlockMod {
    pub fn init(_block: &mut block_coal_block::BlockCoalBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
