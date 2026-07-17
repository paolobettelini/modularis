use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct RedSandBlockItem;
impl Item for RedSandBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:red_sand_block",
        label: "Red Sand",
    };
}

impl ItemRender for RedSandBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-red-sand-block:item/red_sand_block"),
    };
}
pub const ITEM_INFO: ItemInfo = RedSandBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <RedSandBlockItem as ItemRender>::RENDER;
pub struct ItemRedSandBlockMod;
impl ItemRedSandBlockMod {
    pub fn init(_block: &mut block_red_sand::BlockRedSandMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
