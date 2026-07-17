use item_api::{Item, ItemInfo};
use tokio::task::JoinHandle;
pub struct CalciteBlockItem;
impl Item for CalciteBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:calcite_block",
        label: "Calcite",
    };
}
pub const ITEM_INFO: ItemInfo = CalciteBlockItem::INFO;
pub struct ItemCalciteBlockMod;
impl ItemCalciteBlockMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
