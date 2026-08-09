use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct YellowTerracottaBlockItem;

impl Item for YellowTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:yellow_terracotta_block",
        label: "Yellow Terracotta",
    };
}

impl ItemRender for YellowTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-yellow-terracotta-block:item/yellow_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = YellowTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <YellowTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemYellowTerracottaBlockMod;

impl ItemYellowTerracottaBlockMod {
    pub fn init(_block: &mut block_yellow_terracotta::BlockYellowTerracottaMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
