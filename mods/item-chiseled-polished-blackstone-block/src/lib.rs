use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ChiseledPolishedBlackstoneBlockItem;

impl Item for ChiseledPolishedBlackstoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:chiseled_polished_blackstone_block",
        label: "Chiseled Polished Blackstone",
    };
}

impl ItemRender for ChiseledPolishedBlackstoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some(
            "item-chiseled-polished-blackstone-block:item/chiseled_polished_blackstone_block",
        ),
    };
}

pub const ITEM_INFO: ItemInfo = ChiseledPolishedBlackstoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <ChiseledPolishedBlackstoneBlockItem as ItemRender>::RENDER;

pub struct ItemChiseledPolishedBlackstoneBlockMod;

impl ItemChiseledPolishedBlackstoneBlockMod {
    pub fn init(
        _block: &mut block_chiseled_polished_blackstone::BlockChiseledPolishedBlackstoneMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
