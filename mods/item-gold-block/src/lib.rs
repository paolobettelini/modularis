use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GoldBlockItem;

impl Item for GoldBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:gold_block",
        label: "Gold Block",
    };
}

impl ItemRender for GoldBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-gold-block:item/gold_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GoldBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GoldBlockItem as ItemRender>::RENDER;

pub struct ItemGoldBlockMod;

impl ItemGoldBlockMod {
    pub fn init(_block: &mut block_gold_block::BlockGoldBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
