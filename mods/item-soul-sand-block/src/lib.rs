use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct SoulSandBlockItem;
impl Item for SoulSandBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:soul_sand_block",
        label: "Soul Sand",
    };
}

impl ItemRender for SoulSandBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-soul-sand-block:item/soul_sand_block"),
    };
}
pub const ITEM_INFO: ItemInfo = SoulSandBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SoulSandBlockItem as ItemRender>::RENDER;
pub struct ItemSoulSandBlockMod;
impl ItemSoulSandBlockMod {
    pub fn init(_block: &mut block_soul_sand::BlockSoulSandMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
