use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct ObsidianBlockItem;

impl Item for ObsidianBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:obsidian_block",
        label: "Obsidian",
    };
}

impl ItemRender for ObsidianBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-obsidian-block:item/obsidian_block"),
    };
}

pub const ITEM_INFO: ItemInfo = ObsidianBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <ObsidianBlockItem as ItemRender>::RENDER;

pub struct ItemObsidianBlockMod;

impl ItemObsidianBlockMod {
    pub fn init(_block: &mut block_obsidian::BlockObsidianMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
