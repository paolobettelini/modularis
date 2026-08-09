use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DarkPrismarineBlockItem;

impl Item for DarkPrismarineBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:dark_prismarine_block",
        label: "Dark Prismarine",
    };
}

impl ItemRender for DarkPrismarineBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-dark-prismarine-block:item/dark_prismarine_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DarkPrismarineBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DarkPrismarineBlockItem as ItemRender>::RENDER;

pub struct ItemDarkPrismarineBlockMod;

impl ItemDarkPrismarineBlockMod {
    pub fn init(_block: &mut block_dark_prismarine::BlockDarkPrismarineMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
