use bevy::prelude::*;
use inventory_core_api::InventoryCell;

#[derive(Component, Debug, Clone)]
pub struct InventorySlotVisual {
    pub cell: InventoryCell,
}

#[derive(Component, Debug, Clone)]
pub struct InventoryItemVisual {
    pub cell: InventoryCell,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct InventoryItemNameVisual;

pub trait ClientInventoryUiApi: Send + Sync + 'static {}
