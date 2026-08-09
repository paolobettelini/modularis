use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PinkWoolBlockItem;

impl Item for PinkWoolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:pink_wool_block",
        label: "Pink Wool",
    };
}

impl ItemRender for PinkWoolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-pink-wool-block:item/pink_wool_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PinkWoolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PinkWoolBlockItem as ItemRender>::RENDER;

pub struct ItemPinkWoolBlockMod;

impl ItemPinkWoolBlockMod {
    pub fn init(_block: &mut block_pink_wool::BlockPinkWoolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
