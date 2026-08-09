use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct RedSandstoneBlockItem;

impl Item for RedSandstoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:red_sandstone_block",
        label: "Red Sandstone",
    };
}

impl ItemRender for RedSandstoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-red-sandstone-block:item/red_sandstone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = RedSandstoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <RedSandstoneBlockItem as ItemRender>::RENDER;

pub struct ItemRedSandstoneBlockMod;

impl ItemRedSandstoneBlockMod {
    pub fn init(_block: &mut block_red_sandstone::BlockRedSandstoneMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
