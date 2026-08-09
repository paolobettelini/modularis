use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BrownMushroomBlockItem;

impl Item for BrownMushroomBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:brown_mushroom_block",
        label: "Brown Mushroom Block",
    };
}

impl ItemRender for BrownMushroomBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-brown-mushroom-block:item/brown_mushroom_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BrownMushroomBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BrownMushroomBlockItem as ItemRender>::RENDER;

pub struct ItemBrownMushroomBlockMod;

impl ItemBrownMushroomBlockMod {
    pub fn init(_block: &mut block_brown_mushroom_block::BlockBrownMushroomBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
