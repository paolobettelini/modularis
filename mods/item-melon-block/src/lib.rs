use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MelonBlockItem;

impl Item for MelonBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:melon_block",
        label: "Melon",
    };
}

impl ItemRender for MelonBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-melon-block:item/melon_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MelonBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MelonBlockItem as ItemRender>::RENDER;

pub struct ItemMelonBlockMod;

impl ItemMelonBlockMod {
    pub fn init(_block: &mut block_melon::BlockMelonMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
