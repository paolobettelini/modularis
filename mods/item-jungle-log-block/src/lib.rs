use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct JungleLogBlockItem;

impl Item for JungleLogBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:jungle_log_block",
        label: "Jungle Log",
    };
}

impl ItemRender for JungleLogBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-jungle-log-block:item/jungle_log_block"),
    };
}

pub const ITEM_INFO: ItemInfo = JungleLogBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <JungleLogBlockItem as ItemRender>::RENDER;

pub struct ItemJungleLogBlockMod;

impl ItemJungleLogBlockMod {
    pub fn init(_block: &mut block_jungle_log::BlockJungleLogMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
