use bevy::prelude::*;
use inventory_core_api::InventoryCell;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientInventoryUiSet {
    Base,
    Extensions,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct InventoryUiRoot;

#[derive(Component, Debug, Clone, Copy)]
pub struct InventoryMainPanel;

/// Shared input-capture flag for optional inventory UI extensions.
#[derive(Resource, Debug, Default)]
pub struct InventoryUiInputCapture(pub bool);

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
