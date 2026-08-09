use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct AcaciaPlanksBlockItem;

impl Item for AcaciaPlanksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:acacia_planks_block",
        label: "Acacia Planks",
    };
}

impl ItemRender for AcaciaPlanksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-acacia-planks-block:item/acacia_planks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = AcaciaPlanksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <AcaciaPlanksBlockItem as ItemRender>::RENDER;

pub struct ItemAcaciaPlanksBlockMod;

impl ItemAcaciaPlanksBlockMod {
    pub fn init(_block: &mut block_acacia_planks::BlockAcaciaPlanksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
