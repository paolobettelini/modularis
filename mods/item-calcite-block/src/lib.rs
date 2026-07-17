use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;
pub struct CalciteBlockItem;
impl Item for CalciteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:calcite_block",
        label: "Calcite",
    };
}

impl ItemRender for CalciteBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-calcite-block:item/calcite_block"),
    };
}
pub const ITEM_INFO: ItemInfo = CalciteBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CalciteBlockItem as ItemRender>::RENDER;
pub struct ItemCalciteBlockMod;
impl ItemCalciteBlockMod {
    pub fn init(_block: &mut block_calcite::BlockCalciteMod) -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
