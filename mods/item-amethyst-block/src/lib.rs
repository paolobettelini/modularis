use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct AmethystBlockItem;

impl Item for AmethystBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:amethyst_block",
        label: "Amethyst Block",
    };
}

impl ItemRender for AmethystBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-amethyst-block:item/amethyst_block"),
    };
}

pub const ITEM_INFO: ItemInfo = AmethystBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <AmethystBlockItem as ItemRender>::RENDER;

pub struct ItemAmethystBlockMod;

impl ItemAmethystBlockMod {
    pub fn init(_block: &mut block_amethyst_block::BlockAmethystBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
