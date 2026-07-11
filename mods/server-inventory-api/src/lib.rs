use bevy::prelude::*;
use inventory_core_api::{Inventory, InventoryCell, InventoryError, InventoryLayout};
use item_instance_api::ItemInstance;
use std::collections::HashMap;

pub type InventoryOwnerId = u64;

#[derive(Debug, Clone)]
pub struct ServerPlayerInventory {
    pub inventory: Inventory,
    pub selected_hotbar: u32,
}

#[derive(Resource, Default)]
pub struct ServerInventories {
    players: HashMap<InventoryOwnerId, ServerPlayerInventory>,
}

impl ServerInventories {
    pub fn get(&self, player_id: InventoryOwnerId) -> Option<&ServerPlayerInventory> {
        self.players.get(&player_id)
    }

    pub fn get_mut(&mut self, player_id: InventoryOwnerId) -> Option<&mut ServerPlayerInventory> {
        self.players.get_mut(&player_id)
    }

    pub fn reset(
        &mut self,
        player_id: InventoryOwnerId,
        inventory: Inventory,
        selected_hotbar: u32,
    ) {
        self.players.insert(
            player_id,
            ServerPlayerInventory {
                inventory,
                selected_hotbar,
            },
        );
    }

    pub fn remove(&mut self, player_id: InventoryOwnerId) {
        self.players.remove(&player_id);
    }

    pub fn resize(
        &mut self,
        player_id: InventoryOwnerId,
        layout: InventoryLayout,
    ) -> Result<(), InventoryError> {
        self.players
            .get_mut(&player_id)
            .ok_or_else(|| {
                InventoryError::CellOutsideLayout(InventoryCell::new("missing-player", 0))
            })?
            .inventory
            .resize(layout)
    }

    pub fn set_cell(
        &mut self,
        player_id: InventoryOwnerId,
        cell: InventoryCell,
        item: Option<ItemInstance>,
    ) -> Result<Option<ItemInstance>, InventoryError> {
        self.players
            .get_mut(&player_id)
            .ok_or_else(|| InventoryError::CellOutsideLayout(cell.clone()))?
            .inventory
            .set(cell, item)
    }
}

pub trait ServerInventoryApi: Send + Sync + 'static {}
