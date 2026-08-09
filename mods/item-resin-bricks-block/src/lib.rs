use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ResinBricksBlockItem;

impl Item for ResinBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:resin_bricks_block",
        label: "Resin Bricks",
    };
}

impl ItemRender for ResinBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-resin-bricks-block:item/resin_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ResinBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ResinBricksBlockItem as ItemRender>::RENDER;

pub struct ItemResinBricksBlockMod;

impl ItemResinBricksBlockMod {
    pub fn init(_block: &mut block_resin_bricks::BlockResinBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
