use inventory_core_api::InventoryCell;
use item_instance_api::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreativeItemTakeRequest {
    pub operation_id: u64,
    pub item: ItemId,
    pub to: InventoryCell,
}
