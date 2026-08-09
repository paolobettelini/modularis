use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ChiseledQuartzBlockItem;

impl Item for ChiseledQuartzBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:chiseled_quartz_block",
        label: "Chiseled Quartz Block",
    };
}

impl ItemRender for ChiseledQuartzBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-chiseled-quartz-block:item/chiseled_quartz_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ChiseledQuartzBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ChiseledQuartzBlockItem as ItemRender>::RENDER;

pub struct ItemChiseledQuartzBlockMod;

impl ItemChiseledQuartzBlockMod {
    pub fn init(_block: &mut block_chiseled_quartz_block::BlockChiseledQuartzBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
