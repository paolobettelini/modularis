use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DeepslateBricksBlockItem;

impl Item for DeepslateBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:deepslate_bricks_block",
        label: "Deepslate Bricks",
    };
}

impl ItemRender for DeepslateBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-deepslate-bricks-block:item/deepslate_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DeepslateBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DeepslateBricksBlockItem as ItemRender>::RENDER;

pub struct ItemDeepslateBricksBlockMod;

impl ItemDeepslateBricksBlockMod {
    pub fn init(_block: &mut block_deepslate_bricks::BlockDeepslateBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
