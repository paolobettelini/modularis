use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct OrangeWoolBlockItem;

impl Item for OrangeWoolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:orange_wool_block",
        label: "Orange Wool",
    };
}

impl ItemRender for OrangeWoolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-orange-wool-block:item/orange_wool_block"),
    };
}

pub const ITEM_INFO: ItemInfo = OrangeWoolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <OrangeWoolBlockItem as ItemRender>::RENDER;

pub struct ItemOrangeWoolBlockMod;

impl ItemOrangeWoolBlockMod {
    pub fn init(_block: &mut block_orange_wool::BlockOrangeWoolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
