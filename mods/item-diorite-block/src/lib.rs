use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct DioriteBlockItem;

impl Item for DioriteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:diorite_block",
        label: "Diorite",
    };
}

impl ItemRender for DioriteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-diorite-block:item/diorite_block"),
    };
}

pub const ITEM_INFO: ItemInfo = DioriteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <DioriteBlockItem as ItemRender>::RENDER;

pub struct ItemDioriteBlockMod;

impl ItemDioriteBlockMod {
    pub fn init(_block: &mut block_diorite::BlockDioriteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
