use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BrownWoolBlockItem;

impl Item for BrownWoolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:brown_wool_block",
        label: "Brown Wool",
    };
}

impl ItemRender for BrownWoolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-brown-wool-block:item/brown_wool_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BrownWoolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BrownWoolBlockItem as ItemRender>::RENDER;

pub struct ItemBrownWoolBlockMod;

impl ItemBrownWoolBlockMod {
    pub fn init(_block: &mut block_brown_wool::BlockBrownWoolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
