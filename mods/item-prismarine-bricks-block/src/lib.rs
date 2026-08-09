use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PrismarineBricksBlockItem;

impl Item for PrismarineBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:prismarine_bricks_block",
        label: "Prismarine Bricks",
    };
}

impl ItemRender for PrismarineBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-prismarine-bricks-block:item/prismarine_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PrismarineBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PrismarineBricksBlockItem as ItemRender>::RENDER;

pub struct ItemPrismarineBricksBlockMod;

impl ItemPrismarineBricksBlockMod {
    pub fn init(_block: &mut block_prismarine_bricks::BlockPrismarineBricksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
