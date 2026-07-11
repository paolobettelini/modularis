use bevy::prelude::*;
use bevy_mod::BevyMod;
use cell_menu_api::{
    CellMenuMoveEndpoint, LocalCellMenuInventoryMoveIntent, LocalCellMenuMoveIntent,
};
use cell_menu_events_mod::CellMenuEventsMod;
use client_cell_menu_cache_api::{ClientCellMenuCache, ClientCellMenuCacheApi};
use client_inventory_cache_api::{ClientInventoryCache, ClientInventoryCacheApi};
use inventory_core_api::{Inventory, InventoryCell};
use tokio::task::JoinHandle;

pub struct ClientCellMenuOptimisticMoveMod;

impl ClientCellMenuOptimisticMoveMod {
    pub fn init<C: ClientCellMenuCacheApi, I: ClientInventoryCacheApi>(
        bevy: &mut BevyMod,
        _events: &mut CellMenuEventsMod,
        _cell_menu_cache: &mut C,
        _inventory_cache: &mut I,
    ) -> Self {
        bevy.app.add_systems(Update, apply_cell_menu_move_preview);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_cell_menu_move_preview(
    mut cell_menus: ResMut<ClientCellMenuCache>,
    mut inventory_cache: ResMut<ClientInventoryCache>,
    mut moves: MessageReader<LocalCellMenuMoveIntent>,
    mut bridge_moves: MessageReader<LocalCellMenuInventoryMoveIntent>,
) {
    for event in moves.read() {
        let Some(menu) = cell_menus.menus.get_mut(&event.menu_id) else {
            continue;
        };
        if menu
            .inventory
            .move_or_swap(&event.from, &event.to)
            .unwrap_or(false)
        {
            cell_menus.content_revision += 1;
        }
    }

    for event in bridge_moves.read() {
        match (&event.from, &event.to) {
            (
                CellMenuMoveEndpoint::PlayerInventory {
                    cell: inventory_cell,
                },
                CellMenuMoveEndpoint::CellMenu {
                    menu_id,
                    cell: menu_cell,
                },
            ) => {
                if apply_cross_preview(
                    inventory_cell,
                    menu_id,
                    menu_cell,
                    &mut inventory_cache,
                    &mut cell_menus,
                    Direction::InventoryToMenu,
                ) {
                    inventory_cache.content_revision += 1;
                    cell_menus.content_revision += 1;
                }
            }
            (
                CellMenuMoveEndpoint::CellMenu {
                    menu_id,
                    cell: menu_cell,
                },
                CellMenuMoveEndpoint::PlayerInventory {
                    cell: inventory_cell,
                },
            ) => {
                if apply_cross_preview(
                    inventory_cell,
                    menu_id,
                    menu_cell,
                    &mut inventory_cache,
                    &mut cell_menus,
                    Direction::MenuToInventory,
                ) {
                    inventory_cache.content_revision += 1;
                    cell_menus.content_revision += 1;
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    InventoryToMenu,
    MenuToInventory,
}

fn apply_cross_preview(
    inventory_cell: &InventoryCell,
    menu_id: &cell_menu_api::CellMenuId,
    menu_cell: &InventoryCell,
    inventory_cache: &mut ClientInventoryCache,
    cell_menus: &mut ClientCellMenuCache,
    direction: Direction,
) -> bool {
    let Some(inventory) = inventory_cache.inventory.as_mut() else {
        return false;
    };
    let Some(menu) = cell_menus.menus.get_mut(menu_id) else {
        return false;
    };
    match direction {
        Direction::InventoryToMenu => {
            swap_between(inventory, inventory_cell, &mut menu.inventory, menu_cell)
        }
        Direction::MenuToInventory => {
            swap_between(&mut menu.inventory, menu_cell, inventory, inventory_cell)
        }
    }
}

fn swap_between(
    from_inventory: &mut Inventory,
    from_cell: &InventoryCell,
    to_inventory: &mut Inventory,
    to_cell: &InventoryCell,
) -> bool {
    if !from_inventory.layout.contains(from_cell) || !to_inventory.layout.contains(to_cell) {
        return false;
    }
    let Some(source) = from_inventory.get(from_cell).cloned() else {
        return false;
    };
    let target = to_inventory.get(to_cell).cloned();
    if from_inventory.set(from_cell.clone(), target).is_err() {
        return false;
    }
    to_inventory.set(to_cell.clone(), Some(source)).is_ok()
}
