use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct YellowWoolBlockItem;

impl Item for YellowWoolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:yellow_wool_block",
        label: "Yellow Wool",
    };
}

impl ItemRender for YellowWoolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-yellow-wool-block:item/yellow_wool_block"),
    };
}

pub const ITEM_INFO: ItemInfo = YellowWoolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <YellowWoolBlockItem as ItemRender>::RENDER;

pub struct ItemYellowWoolBlockMod;

impl ItemYellowWoolBlockMod {
    pub fn init(_block: &mut block_yellow_wool::BlockYellowWoolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
