use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PrismarineBlockItem;

impl Item for PrismarineBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:prismarine_block",
        label: "Prismarine",
    };
}

impl ItemRender for PrismarineBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-prismarine-block:item/prismarine_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PrismarineBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PrismarineBlockItem as ItemRender>::RENDER;

pub struct ItemPrismarineBlockMod;

impl ItemPrismarineBlockMod {
    pub fn init(_block: &mut block_prismarine::BlockPrismarineMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
