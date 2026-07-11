use cell_menu_api::{CellMenuId, CellMenuMoveEndpoint, CellMenuState};
use inventory_core_api::InventoryCell;
use item_instance_api::ItemInstance;
use serde::{Deserialize, Serialize};
use voxel_math_api::BlockPos;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CellMenuOpenRequest {
    pub kind: String,
    pub anchor: Option<BlockPos>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CellMenuMoveRequest {
    pub operation_id: u64,
    pub menu_id: CellMenuId,
    pub from: InventoryCell,
    pub to: InventoryCell,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CellMenuCloseRequest {
    pub menu_id: CellMenuId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CellMenuInventoryMoveRequest {
    pub operation_id: u64,
    pub from: CellMenuMoveEndpoint,
    pub to: CellMenuMoveEndpoint,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CellMenuOpenPacket {
    pub menu: CellMenuState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CellMenuSetCellPacket {
    pub menu_id: CellMenuId,
    pub cell: InventoryCell,
    pub item: Option<ItemInstance>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CellMenuClosePacket {
    pub menu_id: CellMenuId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CellMenuRequest {
    Open(CellMenuOpenRequest),
    Move(CellMenuMoveRequest),
    InventoryMove(CellMenuInventoryMoveRequest),
    Close(CellMenuCloseRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CellMenuPacket {
    Open(CellMenuOpenPacket),
    SetCell(CellMenuSetCellPacket),
    Close(CellMenuClosePacket),
}
