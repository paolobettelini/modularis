use bevy::prelude::*;
use inventory_core_api::InventoryCell;
use item_instance_api::ItemId;

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

#[derive(Resource, Debug, Default)]
pub struct InventoryOperationSequence(pub u64);

impl InventoryOperationSequence {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(1);
        self.0
    }
}

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

#[derive(Component, Debug, Clone, Copy)]
pub struct ItemCatalogVisual {
    pub item: ItemId,
}

pub trait ClientInventoryUiApi: Send + Sync + 'static {}
