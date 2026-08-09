use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ChiseledResinBricksBlockItem;

impl Item for ChiseledResinBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:chiseled_resin_bricks_block",
        label: "Chiseled Resin Bricks",
    };
}

impl ItemRender for ChiseledResinBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-chiseled-resin-bricks-block:item/chiseled_resin_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ChiseledResinBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ChiseledResinBricksBlockItem as ItemRender>::RENDER;

pub struct ItemChiseledResinBricksBlockMod;

impl ItemChiseledResinBricksBlockMod {
    pub fn init(_block: &mut block_chiseled_resin_bricks::BlockChiseledResinBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
