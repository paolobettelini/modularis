use bevy::prelude::*;
use bevy_mod::BevyMod;
use cell_menu_api::{
    CellMenuCellSet, CellMenuInventoryMoveRequested, CellMenuMoveEndpoint, CellMenuServerSet,
};
use cell_menu_events_mod::CellMenuEventsMod;
use inventory_core_api::{Inventory, InventoryCell};
use inventory_events_api::InventoryCellSet;
use inventory_events_mod::InventoryEventsMod;
use server_cell_menu_api::{ServerCellMenuApi, ServerCellMenus};
use server_inventory_api::{ServerInventories, ServerInventoryApi};
use tokio::task::JoinHandle;

pub struct ServerCellMenuInventoryBridgeMod;

impl ServerCellMenuInventoryBridgeMod {
    pub fn init<C: ServerCellMenuApi, I: ServerInventoryApi>(
        bevy: &mut BevyMod,
        _cell_menu_events: &mut CellMenuEventsMod,
        _inventory_events: &mut InventoryEventsMod,
        _cell_menus: &mut C,
        _inventories: &mut I,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            apply_inventory_cell_menu_moves.in_set(CellMenuServerSet::Apply),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_inventory_cell_menu_moves(
    mut inventories: ResMut<ServerInventories>,
    mut menus: ResMut<ServerCellMenus>,
    mut requests: MessageReader<CellMenuInventoryMoveRequested>,
    mut inventory_cells: MessageWriter<InventoryCellSet>,
    mut menu_cells: MessageWriter<CellMenuCellSet>,
) {
    for request in requests.read() {
        match (&request.from, &request.to) {
            (
                CellMenuMoveEndpoint::PlayerInventory {
                    cell: inventory_cell,
                },
                CellMenuMoveEndpoint::CellMenu {
                    menu_id,
                    cell: menu_cell,
                },
            ) => apply_cross_move(
                request.player_id,
                inventory_cell,
                menu_id,
                menu_cell,
                &mut inventories,
                &mut menus,
                &mut inventory_cells,
                &mut menu_cells,
                Direction::InventoryToMenu,
            ),
            (
                CellMenuMoveEndpoint::CellMenu {
                    menu_id,
                    cell: menu_cell,
                },
                CellMenuMoveEndpoint::PlayerInventory {
                    cell: inventory_cell,
                },
            ) => apply_cross_move(
                request.player_id,
                inventory_cell,
                menu_id,
                menu_cell,
                &mut inventories,
                &mut menus,
                &mut inventory_cells,
                &mut menu_cells,
                Direction::MenuToInventory,
            ),
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    InventoryToMenu,
    MenuToInventory,
}

#[allow(clippy::too_many_arguments)]
fn apply_cross_move(
    player_id: u64,
    inventory_cell: &InventoryCell,
    menu_id: &cell_menu_api::CellMenuId,
    menu_cell: &InventoryCell,
    inventories: &mut ServerInventories,
    menus: &mut ServerCellMenus,
    inventory_cells: &mut MessageWriter<InventoryCellSet>,
    menu_cells: &mut MessageWriter<CellMenuCellSet>,
    direction: Direction,
) {
    let (inventory_item, menu_item) = {
        let Some(player_inventory) = inventories.get_mut(player_id) else {
            return;
        };
        let Some(menu) = menus.menu_mut_for_viewer(player_id, menu_id) else {
            return;
        };

        let moved = match direction {
            Direction::InventoryToMenu => swap_between(
                &mut player_inventory.inventory,
                inventory_cell,
                &mut menu.menu.inventory,
                menu_cell,
            ),
            Direction::MenuToInventory => swap_between(
                &mut menu.menu.inventory,
                menu_cell,
                &mut player_inventory.inventory,
                inventory_cell,
            ),
        };
        if !moved {
            return;
        }
        (
            player_inventory.inventory.get(inventory_cell).cloned(),
            menu.menu.inventory.get(menu_cell).cloned(),
        )
    };

    inventory_cells.write(InventoryCellSet {
        player_id,
        cell: inventory_cell.clone(),
        item: inventory_item,
    });
    for viewer in menus.viewers(menu_id) {
        menu_cells.write(CellMenuCellSet {
            viewer,
            menu_id: menu_id.clone(),
            cell: menu_cell.clone(),
            item: menu_item.clone(),
        });
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
