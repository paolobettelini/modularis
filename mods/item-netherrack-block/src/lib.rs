use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct NetherrackBlockItem;

impl Item for NetherrackBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:netherrack_block",
        label: "Netherrack",
    };
}

impl ItemRender for NetherrackBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-netherrack-block:item/netherrack_block"),
    };
}

pub const ITEM_INFO: ItemInfo = NetherrackBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <NetherrackBlockItem as ItemRender>::RENDER;

pub struct ItemNetherrackBlockMod;

impl ItemNetherrackBlockMod {
    pub fn init(_block: &mut block_netherrack::BlockNetherrackMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
