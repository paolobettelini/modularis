use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PolishedDioriteBlockItem;

impl Item for PolishedDioriteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:polished_diorite_block",
        label: "Polished Diorite",
    };
}

impl ItemRender for PolishedDioriteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-polished-diorite-block:item/polished_diorite_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PolishedDioriteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PolishedDioriteBlockItem as ItemRender>::RENDER;

pub struct ItemPolishedDioriteBlockMod;

impl ItemPolishedDioriteBlockMod {
    pub fn init(_block: &mut block_polished_diorite::BlockPolishedDioriteMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
