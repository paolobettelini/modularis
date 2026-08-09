use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct SandstoneBlockItem;

impl Item for SandstoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:sandstone_block",
        label: "Sandstone",
    };
}

impl ItemRender for SandstoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-sandstone-block:item/sandstone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = SandstoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <SandstoneBlockItem as ItemRender>::RENDER;

pub struct ItemSandstoneBlockMod;

impl ItemSandstoneBlockMod {
    pub fn init(_block: &mut block_sandstone::BlockSandstoneMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
