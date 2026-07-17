use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct SandBlockItem;
impl Item for SandBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:sand_block",
        label: "Sand",
    };
}

impl ItemRender for SandBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-sand-block:item/sand_block"),
    };
}
pub const ITEM_INFO: ItemInfo = SandBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SandBlockItem as ItemRender>::RENDER;
pub struct ItemSandBlockMod;
impl ItemSandBlockMod {
    pub fn init(_block: &mut block_sand::BlockSandMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
