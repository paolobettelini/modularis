use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct NetheriteBlockItem;

impl Item for NetheriteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:netherite_block",
        label: "Netherite Block",
    };
}

impl ItemRender for NetheriteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-netherite-block:item/netherite_block"),
    };
}

pub const ITEM_INFO: ItemInfo = NetheriteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <NetheriteBlockItem as ItemRender>::RENDER;

pub struct ItemNetheriteBlockMod;

impl ItemNetheriteBlockMod {
    pub fn init(_block: &mut block_netherite_block::BlockNetheriteBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
