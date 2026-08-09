use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GraniteBlockItem;

impl Item for GraniteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:granite_block",
        label: "Granite",
    };
}

impl ItemRender for GraniteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-granite-block:item/granite_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GraniteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GraniteBlockItem as ItemRender>::RENDER;

pub struct ItemGraniteBlockMod;

impl ItemGraniteBlockMod {
    pub fn init(_block: &mut block_granite::BlockGraniteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
