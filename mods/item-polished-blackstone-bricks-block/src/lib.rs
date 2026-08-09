use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PolishedBlackstoneBricksBlockItem;

impl Item for PolishedBlackstoneBricksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:polished_blackstone_bricks_block",
        label: "Polished Blackstone Bricks",
    };
}

impl ItemRender for PolishedBlackstoneBricksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-polished-blackstone-bricks-block:item/polished_blackstone_bricks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PolishedBlackstoneBricksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <PolishedBlackstoneBricksBlockItem as ItemRender>::RENDER;

pub struct ItemPolishedBlackstoneBricksBlockMod;

impl ItemPolishedBlackstoneBricksBlockMod {
    pub fn init(
        _block: &mut block_polished_blackstone_bricks::BlockPolishedBlackstoneBricksMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
