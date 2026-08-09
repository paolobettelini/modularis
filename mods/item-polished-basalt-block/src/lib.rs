use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PolishedBasaltBlockItem;

impl Item for PolishedBasaltBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:polished_basalt_block",
        label: "Polished Basalt",
    };
}

impl ItemRender for PolishedBasaltBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-polished-basalt-block:item/polished_basalt_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PolishedBasaltBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PolishedBasaltBlockItem as ItemRender>::RENDER;

pub struct ItemPolishedBasaltBlockMod;

impl ItemPolishedBasaltBlockMod {
    pub fn init(_block: &mut block_polished_basalt::BlockPolishedBasaltMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
