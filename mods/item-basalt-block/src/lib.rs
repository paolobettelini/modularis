use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct BasaltBlockItem;
impl Item for BasaltBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:basalt_block",
        label: "Basalt",
    };
}

impl ItemRender for BasaltBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-basalt-block:item/basalt_block"),
    };
}
pub const ITEM_INFO: ItemInfo = BasaltBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BasaltBlockItem as ItemRender>::RENDER;
pub struct ItemBasaltBlockMod;
impl ItemBasaltBlockMod {
    pub fn init(_block: &mut block_basalt::BlockBasaltMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
