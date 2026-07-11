use bevy::prelude::*;
use inventory_core_api::{Inventory, InventoryCell, InventoryLayout};
use item_instance_api::ItemInstance;
use item_use_api::ItemUseTarget;

pub type InventoryOwnerId = u64;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InventoryServerSet {
    ReceiveRequest,
    Validate,
    DispatchUse,
    ApplyWorldEffects,
    ApplyConsumption,
    Sync,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InventoryValidationSet {
    Initialize,
    Stack,
    MoveOrSwap,
    Other,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InventoryClientSet {
    ReceiveSync,
    ApplyCache,
    Render,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InventoryClientCacheSet {
    AuthoritativeSync,
    OptimisticPreview,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InventoryClientRenderSet {
    Layout,
    Decorations,
}

#[derive(Message, Debug, Clone)]
pub struct LocalInventoryMoveIntent {
    pub operation_id: u64,
    pub from: InventoryCell,
    pub to: InventoryCell,
}

#[derive(Message, Debug, Clone)]
pub struct LocalHotbarSelectIntent {
    pub index: u32,
}

#[derive(Message, Debug, Clone)]
pub struct LocalUseHeldItemIntent {
    pub target: ItemUseTarget,
}

#[derive(Message, Debug, Clone)]
pub struct InventoryMoveRequested {
    pub operation_id: u64,
    pub player_id: InventoryOwnerId,
    pub from: InventoryCell,
    pub to: InventoryCell,
}

#[derive(Message, Debug, Clone)]
pub struct InventoryMoveHandled {
    pub operation_id: u64,
    pub player_id: InventoryOwnerId,
}

#[derive(Message, Debug, Clone)]
pub struct HotbarSelectRequested {
    pub player_id: InventoryOwnerId,
    pub index: u32,
}

#[derive(Message, Debug, Clone)]
pub struct UseHeldItemRequested {
    pub player_id: InventoryOwnerId,
    pub target: ItemUseTarget,
}

#[derive(Message, Debug, Clone)]
pub struct InventorySyncRequested {
    pub player_id: InventoryOwnerId,
}

#[derive(Message, Debug, Clone)]
pub struct InventoryResetRequested {
    pub player_id: InventoryOwnerId,
    pub inventory: Inventory,
    pub selected_hotbar: u32,
}

#[derive(Message, Debug, Clone)]
pub struct InventoryResizeRequested {
    pub player_id: InventoryOwnerId,
    pub layout: InventoryLayout,
}

#[derive(Message, Debug, Clone)]
pub struct InventorySetCellRequested {
    pub player_id: InventoryOwnerId,
    pub cell: InventoryCell,
    pub item: Option<ItemInstance>,
}

#[derive(Message, Debug, Clone)]
pub struct InventoryResetApplied {
    pub player_id: InventoryOwnerId,
    pub inventory: Inventory,
    pub selected_hotbar: u32,
}

#[derive(Message, Debug, Clone)]
pub struct InventoryResized {
    pub player_id: InventoryOwnerId,
    pub layout: InventoryLayout,
}

#[derive(Message, Debug, Clone)]
pub struct InventoryCellSet {
    pub player_id: InventoryOwnerId,
    pub cell: InventoryCell,
    pub item: Option<ItemInstance>,
}

#[derive(Message, Debug, Clone)]
pub struct HotbarSelectionSet {
    pub player_id: InventoryOwnerId,
    pub index: u32,
}

#[derive(Message, Debug, Clone)]
pub struct HeldItemUseDispatched {
    pub player_id: InventoryOwnerId,
    pub cell: InventoryCell,
    pub item: ItemInstance,
    pub target: ItemUseTarget,
}

#[derive(Message, Debug, Clone)]
pub struct ItemUseSucceeded {
    pub player_id: InventoryOwnerId,
    pub cell: InventoryCell,
    pub item_before_use: ItemInstance,
}

#[derive(Message, Debug, Clone)]
pub struct ClientInventoryReset {
    pub inventory: Inventory,
    pub selected_hotbar: u32,
}

#[derive(Message, Debug, Clone)]
pub struct ClientInventoryResized {
    pub layout: InventoryLayout,
}

#[derive(Message, Debug, Clone)]
pub struct ClientInventoryCellSet {
    pub cell: InventoryCell,
    pub item: Option<ItemInstance>,
}

#[derive(Message, Debug, Clone)]
pub struct ClientHotbarSelectionSet {
    pub index: u32,
}

#[derive(Message, Debug, Clone)]
pub struct InventorySlotVisualCreated {
    pub entity: Entity,
    pub item: ItemInstance,
}
