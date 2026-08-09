use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct RootedDirtBlockItem;

impl Item for RootedDirtBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:rooted_dirt_block",
        label: "Rooted Dirt",
    };
}

impl ItemRender for RootedDirtBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-rooted-dirt-block:item/rooted_dirt_block"),
    };
}

pub const ITEM_INFO: ItemInfo = RootedDirtBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <RootedDirtBlockItem as ItemRender>::RENDER;

pub struct ItemRootedDirtBlockMod;

impl ItemRootedDirtBlockMod {
    pub fn init(_block: &mut block_rooted_dirt::BlockRootedDirtMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
