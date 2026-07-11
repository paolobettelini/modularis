use bevy::prelude::*;
use cell_menu_api::CellMenuId;
use inventory_core_api::InventoryCell;

#[derive(Component, Debug, Clone)]
pub struct CellMenuSlotVisual {
    pub menu_id: CellMenuId,
    pub cell: InventoryCell,
}

#[derive(Component, Debug, Clone)]
pub struct CellMenuItemVisual {
    pub menu_id: CellMenuId,
    pub cell: InventoryCell,
}

pub trait ClientCellMenuUiApi: Send + Sync + 'static {}
