use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct AnvilBlockItem;

impl Item for AnvilBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:anvil_block",
        label: "Anvil",
    };
}

impl ItemRender for AnvilBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-anvil-block:item/anvil_block"),
    };
}

pub const ITEM_INFO: ItemInfo = AnvilBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <AnvilBlockItem as ItemRender>::RENDER;

pub struct ItemAnvilBlockMod;

impl ItemAnvilBlockMod {
    pub fn init(_block: &mut block_anvil::BlockAnvilMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
