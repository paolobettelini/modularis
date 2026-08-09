use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ChiseledSulfurBlockItem;

impl Item for ChiseledSulfurBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:chiseled_sulfur_block",
        label: "Chiseled Sulfur",
    };
}

impl ItemRender for ChiseledSulfurBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-chiseled-sulfur-block:item/chiseled_sulfur_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ChiseledSulfurBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ChiseledSulfurBlockItem as ItemRender>::RENDER;

pub struct ItemChiseledSulfurBlockMod;

impl ItemChiseledSulfurBlockMod {
    pub fn init(_block: &mut block_chiseled_sulfur::BlockChiseledSulfurMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
