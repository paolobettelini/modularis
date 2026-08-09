use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct SmoothStoneBlockItem;

impl Item for SmoothStoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:smooth_stone_block",
        label: "Smooth Stone",
    };
}

impl ItemRender for SmoothStoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-smooth-stone-block:item/smooth_stone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = SmoothStoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SmoothStoneBlockItem as ItemRender>::RENDER;

pub struct ItemSmoothStoneBlockMod;

impl ItemSmoothStoneBlockMod {
    pub fn init(_block: &mut block_smooth_stone::BlockSmoothStoneMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
