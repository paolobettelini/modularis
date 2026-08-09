use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PurpleWoolBlockItem;

impl Item for PurpleWoolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:purple_wool_block",
        label: "Purple Wool",
    };
}

impl ItemRender for PurpleWoolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-purple-wool-block:item/purple_wool_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PurpleWoolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PurpleWoolBlockItem as ItemRender>::RENDER;

pub struct ItemPurpleWoolBlockMod;

impl ItemPurpleWoolBlockMod {
    pub fn init(_block: &mut block_purple_wool::BlockPurpleWoolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
