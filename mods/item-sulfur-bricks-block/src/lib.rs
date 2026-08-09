use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct SulfurBricksBlockItem;

impl Item for SulfurBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:sulfur_bricks_block",
        label: "Sulfur Bricks",
    };
}

impl ItemRender for SulfurBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-sulfur-bricks-block:item/sulfur_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = SulfurBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SulfurBricksBlockItem as ItemRender>::RENDER;

pub struct ItemSulfurBricksBlockMod;

impl ItemSulfurBricksBlockMod {
    pub fn init(_block: &mut block_sulfur_bricks::BlockSulfurBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
