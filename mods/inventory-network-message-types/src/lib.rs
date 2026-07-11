use inventory_core_api::{Inventory, InventoryCell, InventoryLayout};
use item_instance_api::ItemInstance;
use item_use_api::ItemUseTarget;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InventoryMoveRequest {
    pub operation_id: u64,
    pub from: InventoryCell,
    pub to: InventoryCell,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HotbarSelectRequest {
    pub index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UseHeldItemRequest {
    pub target: ItemUseTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InventorySyncRequest;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InventoryResetPacket {
    pub inventory: Inventory,
    pub selected_hotbar: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InventoryResizePacket {
    pub layout: InventoryLayout,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InventorySetCellPacket {
    pub cell: InventoryCell,
    pub item: Option<ItemInstance>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HotbarSelectionPacket {
    pub index: u32,
}
