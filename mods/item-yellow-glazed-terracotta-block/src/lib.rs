use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct YellowGlazedTerracottaBlockItem;

impl Item for YellowGlazedTerracottaBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:yellow_glazed_terracotta_block",
        label: "Yellow Glazed Terracotta",
    };
}

impl ItemRender for YellowGlazedTerracottaBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-yellow-glazed-terracotta-block:item/yellow_glazed_terracotta_block"),
    };
}

pub const ITEM_INFO: ItemInfo = YellowGlazedTerracottaBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <YellowGlazedTerracottaBlockItem as ItemRender>::RENDER;

pub struct ItemYellowGlazedTerracottaBlockMod;

impl ItemYellowGlazedTerracottaBlockMod {
    pub fn init(
        _block: &mut block_yellow_glazed_terracotta::BlockYellowGlazedTerracottaMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
