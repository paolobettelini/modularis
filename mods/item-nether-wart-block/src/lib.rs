use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct NetherWartBlockItem;

impl Item for NetherWartBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:nether_wart_block",
        label: "Nether Wart Block",
    };
}

impl ItemRender for NetherWartBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-nether-wart-block:item/nether_wart_block"),
    };
}

pub const ITEM_INFO: ItemInfo = NetherWartBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <NetherWartBlockItem as ItemRender>::RENDER;

pub struct ItemNetherWartBlockMod;

impl ItemNetherWartBlockMod {
    pub fn init(_block: &mut block_nether_wart_block::BlockNetherWartBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
