use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct MossBlockItem;
impl Item for MossBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:moss_block",
        label: "Moss",
    };
}

impl ItemRender for MossBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-moss-block:item/moss_block"),
    };
}
pub const ITEM_INFO: ItemInfo = MossBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MossBlockItem as ItemRender>::RENDER;
pub struct ItemMossBlockMod;
impl ItemMossBlockMod {
    pub fn init(_block: &mut block_moss::BlockMossMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
