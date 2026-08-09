use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PolishedBlackstoneBlockItem;

impl Item for PolishedBlackstoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:polished_blackstone_block",
        label: "Polished Blackstone",
    };
}

impl ItemRender for PolishedBlackstoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-polished-blackstone-block:item/polished_blackstone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PolishedBlackstoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PolishedBlackstoneBlockItem as ItemRender>::RENDER;

pub struct ItemPolishedBlackstoneBlockMod;

impl ItemPolishedBlackstoneBlockMod {
    pub fn init(_block: &mut block_polished_blackstone::BlockPolishedBlackstoneMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
