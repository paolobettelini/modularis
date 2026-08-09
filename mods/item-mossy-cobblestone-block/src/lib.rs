use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MossyCobblestoneBlockItem;

impl Item for MossyCobblestoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:mossy_cobblestone_block",
        label: "Mossy Cobblestone",
    };
}

impl ItemRender for MossyCobblestoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-mossy-cobblestone-block:item/mossy_cobblestone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MossyCobblestoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MossyCobblestoneBlockItem as ItemRender>::RENDER;

pub struct ItemMossyCobblestoneBlockMod;

impl ItemMossyCobblestoneBlockMod {
    pub fn init(_block: &mut block_mossy_cobblestone::BlockMossyCobblestoneMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
