use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MangroveRootsBlockItem;

impl Item for MangroveRootsBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:mangrove_roots_block",
        label: "Mangrove Roots",
    };
}

impl ItemRender for MangroveRootsBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-mangrove-roots-block:item/mangrove_roots_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MangroveRootsBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MangroveRootsBlockItem as ItemRender>::RENDER;

pub struct ItemMangroveRootsBlockMod;

impl ItemMangroveRootsBlockMod {
    pub fn init(_block: &mut block_mangrove_roots::BlockMangroveRootsMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
