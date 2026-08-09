use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct GildedBlackstoneBlockItem;

impl Item for GildedBlackstoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:gilded_blackstone_block",
        label: "Gilded Blackstone",
    };
}

impl ItemRender for GildedBlackstoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-gilded-blackstone-block:item/gilded_blackstone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = GildedBlackstoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <GildedBlackstoneBlockItem as ItemRender>::RENDER;

pub struct ItemGildedBlackstoneBlockMod;

impl ItemGildedBlackstoneBlockMod {
    pub fn init(_block: &mut block_gilded_blackstone::BlockGildedBlackstoneMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
