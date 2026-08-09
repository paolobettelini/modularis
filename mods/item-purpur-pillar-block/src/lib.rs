use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PurpurPillarBlockItem;

impl Item for PurpurPillarBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:purpur_pillar_block",
        label: "Purpur Pillar",
    };
}

impl ItemRender for PurpurPillarBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-purpur-pillar-block:item/purpur_pillar_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PurpurPillarBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PurpurPillarBlockItem as ItemRender>::RENDER;

pub struct ItemPurpurPillarBlockMod;

impl ItemPurpurPillarBlockMod {
    pub fn init(_block: &mut block_purpur_pillar::BlockPurpurPillarMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
