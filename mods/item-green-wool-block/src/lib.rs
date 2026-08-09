use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GreenWoolBlockItem;

impl Item for GreenWoolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:green_wool_block",
        label: "Green Wool",
    };
}

impl ItemRender for GreenWoolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-green-wool-block:item/green_wool_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GreenWoolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GreenWoolBlockItem as ItemRender>::RENDER;

pub struct ItemGreenWoolBlockMod;

impl ItemGreenWoolBlockMod {
    pub fn init(_block: &mut block_green_wool::BlockGreenWoolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
