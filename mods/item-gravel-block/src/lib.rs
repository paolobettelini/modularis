use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct GravelBlockItem;
impl Item for GravelBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:gravel_block",
        label: "Gravel",
    };
}

impl ItemRender for GravelBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-gravel-block:item/gravel_block"),
    };
}
pub const ITEM_INFO: ItemInfo = GravelBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GravelBlockItem as ItemRender>::RENDER;
pub struct ItemGravelBlockMod;
impl ItemGravelBlockMod {
    pub fn init(_block: &mut block_gravel::BlockGravelMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
