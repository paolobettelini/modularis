use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PaleMossBlockItem;

impl Item for PaleMossBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:pale_moss_block",
        label: "Pale Moss Block",
    };
}

impl ItemRender for PaleMossBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-pale-moss-block:item/pale_moss_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PaleMossBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PaleMossBlockItem as ItemRender>::RENDER;

pub struct ItemPaleMossBlockMod;

impl ItemPaleMossBlockMod {
    pub fn init(_block: &mut block_pale_moss_block::BlockPaleMossBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
