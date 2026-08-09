use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct PumpkinBlockItem;

impl Item for PumpkinBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:pumpkin_block",
        label: "Pumpkin",
    };
}

impl ItemRender for PumpkinBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-pumpkin-block:item/pumpkin_block"),
    };
}

pub const ITEM_INFO: ItemInfo = PumpkinBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <PumpkinBlockItem as ItemRender>::RENDER;

pub struct ItemPumpkinBlockMod;

impl ItemPumpkinBlockMod {
    pub fn init(_block: &mut block_pumpkin::BlockPumpkinMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
