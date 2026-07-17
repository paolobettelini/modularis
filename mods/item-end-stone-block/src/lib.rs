use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct EndStoneBlockItem;

impl Item for EndStoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:end_stone_block",
        label: "End Stone",
    };
}

impl ItemRender for EndStoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-end-stone-block:item/end_stone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = EndStoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <EndStoneBlockItem as ItemRender>::RENDER;

pub struct ItemEndStoneBlockMod;

impl ItemEndStoneBlockMod {
    pub fn init(_block: &mut block_end_stone::BlockEndStoneMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
