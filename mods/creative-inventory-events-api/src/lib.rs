use bevy::prelude::*;
use inventory_core_api::InventoryCell;
use item_instance_api::ItemId;

#[derive(Message, Debug, Clone)]
pub struct LocalCreativeItemTakeIntent {
    pub operation_id: u64,
    pub item: ItemId,
    pub to: InventoryCell,
}

#[derive(Message, Debug, Clone)]
pub struct CreativeItemTakeRequested {
    pub operation_id: u64,
    pub player_id: u64,
    pub item: ItemId,
    pub to: InventoryCell,
}
