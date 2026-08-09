use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GrayWoolBlockItem;

impl Item for GrayWoolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:gray_wool_block",
        label: "Gray Wool",
    };
}

impl ItemRender for GrayWoolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-gray-wool-block:item/gray_wool_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GrayWoolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GrayWoolBlockItem as ItemRender>::RENDER;

pub struct ItemGrayWoolBlockMod;

impl ItemGrayWoolBlockMod {
    pub fn init(_block: &mut block_gray_wool::BlockGrayWoolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
