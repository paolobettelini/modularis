use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ChiseledSandstoneBlockItem;

impl Item for ChiseledSandstoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:chiseled_sandstone_block",
        label: "Chiseled Sandstone",
    };
}

impl ItemRender for ChiseledSandstoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-chiseled-sandstone-block:item/chiseled_sandstone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ChiseledSandstoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ChiseledSandstoneBlockItem as ItemRender>::RENDER;

pub struct ItemChiseledSandstoneBlockMod;

impl ItemChiseledSandstoneBlockMod {
    pub fn init(_block: &mut block_chiseled_sandstone::BlockChiseledSandstoneMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
