use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct BlackstoneBlockItem;
impl Item for BlackstoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:blackstone_block",
        label: "Blackstone",
    };
}

impl ItemRender for BlackstoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-blackstone-block:item/blackstone_block"),
    };
}
pub const ITEM_INFO: ItemInfo = BlackstoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BlackstoneBlockItem as ItemRender>::RENDER;
pub struct ItemBlackstoneBlockMod;
impl ItemBlackstoneBlockMod {
    pub fn init(_block: &mut block_blackstone::BlockBlackstoneMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
