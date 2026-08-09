use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MossyStoneBricksBlockItem;

impl Item for MossyStoneBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:mossy_stone_bricks_block",
        label: "Mossy Stone Bricks",
    };
}

impl ItemRender for MossyStoneBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-mossy-stone-bricks-block:item/mossy_stone_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MossyStoneBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MossyStoneBricksBlockItem as ItemRender>::RENDER;

pub struct ItemMossyStoneBricksBlockMod;

impl ItemMossyStoneBricksBlockMod {
    pub fn init(_block: &mut block_mossy_stone_bricks::BlockMossyStoneBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
