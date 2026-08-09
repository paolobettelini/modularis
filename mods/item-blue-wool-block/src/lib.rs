use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BlueWoolBlockItem;

impl Item for BlueWoolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:blue_wool_block",
        label: "Blue Wool",
    };
}

impl ItemRender for BlueWoolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-blue-wool-block:item/blue_wool_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BlueWoolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BlueWoolBlockItem as ItemRender>::RENDER;

pub struct ItemBlueWoolBlockMod;

impl ItemBlueWoolBlockMod {
    pub fn init(_block: &mut block_blue_wool::BlockBlueWoolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
