use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ChiseledNetherBricksBlockItem;

impl Item for ChiseledNetherBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:chiseled_nether_bricks_block",
        label: "Chiseled Nether Bricks",
    };
}

impl ItemRender for ChiseledNetherBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-chiseled-nether-bricks-block:item/chiseled_nether_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ChiseledNetherBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ChiseledNetherBricksBlockItem as ItemRender>::RENDER;

pub struct ItemChiseledNetherBricksBlockMod;

impl ItemChiseledNetherBricksBlockMod {
    pub fn init(_block: &mut block_chiseled_nether_bricks::BlockChiseledNetherBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
