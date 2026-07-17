use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct StoneBlockItem;

impl Item for StoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:stone_block",
        label: "Stone",
    };
}

impl ItemRender for StoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-stone-block:item/stone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = StoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <StoneBlockItem as ItemRender>::RENDER;

pub struct ItemStoneBlockMod;

impl ItemStoneBlockMod {
    pub fn init(_block: &mut block_stone::BlockStoneMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
