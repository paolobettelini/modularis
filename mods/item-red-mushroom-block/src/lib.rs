use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct RedMushroomBlockItem;

impl Item for RedMushroomBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:red_mushroom_block",
        label: "Red Mushroom Block",
    };
}

impl ItemRender for RedMushroomBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-red-mushroom-block:item/red_mushroom_block"),
    };
}

pub const ITEM_INFO: ItemInfo = RedMushroomBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <RedMushroomBlockItem as ItemRender>::RENDER;

pub struct ItemRedMushroomBlockMod;

impl ItemRedMushroomBlockMod {
    pub fn init(_block: &mut block_red_mushroom_block::BlockRedMushroomBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
