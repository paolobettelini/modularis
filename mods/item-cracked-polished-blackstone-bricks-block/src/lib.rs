use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CrackedPolishedBlackstoneBricksBlockItem;

impl Item for CrackedPolishedBlackstoneBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cracked_polished_blackstone_bricks_block",
        label: "Cracked Polished Blackstone Bricks",
    };
}

impl ItemRender for CrackedPolishedBlackstoneBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some(
            "item-cracked-polished-blackstone-bricks-block:item/cracked_polished_blackstone_bricks_block",
        ),
    };
}

pub const ITEM_INFO: ItemInfo = CrackedPolishedBlackstoneBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <CrackedPolishedBlackstoneBricksBlockItem as ItemRender>::RENDER;

pub struct ItemCrackedPolishedBlackstoneBricksBlockMod;

impl ItemCrackedPolishedBlackstoneBricksBlockMod {
    pub fn init(
        _block: &mut block_cracked_polished_blackstone_bricks::BlockCrackedPolishedBlackstoneBricksMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
