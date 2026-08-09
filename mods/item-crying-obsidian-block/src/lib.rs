use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CryingObsidianBlockItem;

impl Item for CryingObsidianBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:crying_obsidian_block",
        label: "Crying Obsidian",
    };
}

impl ItemRender for CryingObsidianBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-crying-obsidian-block:item/crying_obsidian_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CryingObsidianBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CryingObsidianBlockItem as ItemRender>::RENDER;

pub struct ItemCryingObsidianBlockMod;

impl ItemCryingObsidianBlockMod {
    pub fn init(_block: &mut block_crying_obsidian::BlockCryingObsidianMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
