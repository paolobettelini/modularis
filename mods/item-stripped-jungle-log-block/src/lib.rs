use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct StrippedJungleLogBlockItem;

impl Item for StrippedJungleLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:stripped_jungle_log_block",
        label: "Stripped Jungle Log",
    };
}

impl ItemRender for StrippedJungleLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-stripped-jungle-log-block:item/stripped_jungle_log_block"),
    };
}

pub const ITEM_INFO: ItemInfo = StrippedJungleLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <StrippedJungleLogBlockItem as ItemRender>::RENDER;

pub struct ItemStrippedJungleLogBlockMod;

impl ItemStrippedJungleLogBlockMod {
    pub fn init(_block: &mut block_stripped_jungle_log::BlockStrippedJungleLogMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
