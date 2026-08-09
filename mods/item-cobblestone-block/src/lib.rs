use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CobblestoneBlockItem;

impl Item for CobblestoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:cobblestone_block",
        label: "Cobblestone",
    };
}

impl ItemRender for CobblestoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-cobblestone-block:item/cobblestone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CobblestoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CobblestoneBlockItem as ItemRender>::RENDER;

pub struct ItemCobblestoneBlockMod;

impl ItemCobblestoneBlockMod {
    pub fn init(_block: &mut block_cobblestone::BlockCobblestoneMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
