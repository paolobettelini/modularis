use audience_api::{Audience, AudienceMemberId};
use bevy::prelude::*;
use inventory_core_api::{Inventory, InventoryCell, InventoryLayout};
use item_instance_api::ItemInstance;
use serde::{Deserialize, Serialize};

pub type CellMenuViewerId = AudienceMemberId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellMenuId(pub String);

impl CellMenuId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellMenuState {
    pub id: CellMenuId,
    pub title: String,
    pub audience: Audience,
    pub inventory: Inventory,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellMenuServerSet {
    ReceiveRequest,
    Validate,
    Apply,
    Sync,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellMenuClientSet {
    ReceiveSync,
    ApplyCache,
    Render,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellMenuClientRenderSet {
    Layout,
    Decorations,
}

/// Transport-independent request to resolve a named menu behavior.
/// Feature mods decide whether the kind/anchor is valid and emit
/// `CellMenuOpenRequested` only after applying their own policy.
#[derive(Message, Debug, Clone)]
pub struct CellMenuOpenIntent {
    pub player_id: CellMenuViewerId,
    pub kind: String,
    pub anchor: Option<voxel_math_api::BlockPos>,
}

#[derive(Message, Debug, Clone)]
pub struct LocalCellMenuMoveIntent {
    pub operation_id: u64,
    pub menu_id: CellMenuId,
    pub from: InventoryCell,
    pub to: InventoryCell,
}

#[derive(Message, Debug, Clone)]
pub struct LocalCellMenuCloseIntent {
    pub menu_id: CellMenuId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellMenuMoveEndpoint {
    PlayerInventory {
        cell: InventoryCell,
    },
    CellMenu {
        menu_id: CellMenuId,
        cell: InventoryCell,
    },
}

#[derive(Message, Debug, Clone)]
pub struct LocalCellMenuInventoryMoveIntent {
    pub operation_id: u64,
    pub from: CellMenuMoveEndpoint,
    pub to: CellMenuMoveEndpoint,
}

#[derive(Message, Debug, Clone)]
pub struct CellMenuOpenRequested {
    pub player_id: CellMenuViewerId,
    pub menu_id: CellMenuId,
    pub title: String,
    pub audience: Audience,
    pub layout: InventoryLayout,
}

#[derive(Message, Debug, Clone)]
pub struct CellMenuMoveRequested {
    pub operation_id: u64,
    pub player_id: CellMenuViewerId,
    pub menu_id: CellMenuId,
    pub from: InventoryCell,
    pub to: InventoryCell,
}

#[derive(Message, Debug, Clone)]
pub struct CellMenuInventoryMoveRequested {
    pub operation_id: u64,
    pub player_id: CellMenuViewerId,
    pub from: CellMenuMoveEndpoint,
    pub to: CellMenuMoveEndpoint,
}

#[derive(Message, Debug, Clone)]
pub struct CellMenuCloseRequested {
    pub player_id: CellMenuViewerId,
    pub menu_id: CellMenuId,
}

#[derive(Message, Debug, Clone)]
pub struct CellMenuOpened {
    pub viewer: CellMenuViewerId,
    pub menu: CellMenuState,
}

#[derive(Message, Debug, Clone)]
pub struct CellMenuClosed {
    pub viewer: CellMenuViewerId,
    pub menu_id: CellMenuId,
}

#[derive(Message, Debug, Clone)]
pub struct CellMenuCellSet {
    pub viewer: CellMenuViewerId,
    pub menu_id: CellMenuId,
    pub cell: InventoryCell,
    pub item: Option<ItemInstance>,
}

#[derive(Message, Debug, Clone)]
pub struct ClientCellMenuOpened {
    pub menu: CellMenuState,
}

#[derive(Message, Debug, Clone)]
pub struct ClientCellMenuClosed {
    pub menu_id: CellMenuId,
}

#[derive(Message, Debug, Clone)]
pub struct ClientCellMenuCellSet {
    pub menu_id: CellMenuId,
    pub cell: InventoryCell,
    pub item: Option<ItemInstance>,
}
