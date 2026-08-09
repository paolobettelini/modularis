use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PodzolBlockItem;

impl Item for PodzolBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:podzol_block",
        label: "Podzol",
    };
}

impl ItemRender for PodzolBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-podzol-block:item/podzol_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PodzolBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PodzolBlockItem as ItemRender>::RENDER;

pub struct ItemPodzolBlockMod;

impl ItemPodzolBlockMod {
    pub fn init(_block: &mut block_podzol::BlockPodzolMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
