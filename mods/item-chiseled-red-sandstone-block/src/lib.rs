use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ChiseledRedSandstoneBlockItem;

impl Item for ChiseledRedSandstoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:chiseled_red_sandstone_block",
        label: "Chiseled Red Sandstone",
    };
}

impl ItemRender for ChiseledRedSandstoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-chiseled-red-sandstone-block:item/chiseled_red_sandstone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ChiseledRedSandstoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ChiseledRedSandstoneBlockItem as ItemRender>::RENDER;

pub struct ItemChiseledRedSandstoneBlockMod;

impl ItemChiseledRedSandstoneBlockMod {
    pub fn init(_block: &mut block_chiseled_red_sandstone::BlockChiseledRedSandstoneMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
