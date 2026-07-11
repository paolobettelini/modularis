use bevy::prelude::*;
use bevy_mod::BevyMod;
use cell_menu_api::*;
use cell_menu_events_mod::CellMenuEventsMod;
use server_cell_menu_api::{ServerCellMenuApi, ServerCellMenus};
use tokio::task::JoinHandle;

pub struct ServerCellMenuAuthoritativeMod;

impl ServerCellMenuAuthoritativeMod {
    pub fn init(bevy: &mut BevyMod, _events: &mut CellMenuEventsMod) -> Self {
        bevy.app
            .init_resource::<ServerCellMenus>()
            .add_systems(Update, apply_open_requests.in_set(CellMenuServerSet::Apply))
            .add_systems(Update, apply_move_requests.in_set(CellMenuServerSet::Apply))
            .add_systems(
                Update,
                apply_close_requests.in_set(CellMenuServerSet::Apply),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerCellMenuApi for ServerCellMenuAuthoritativeMod {}

fn apply_open_requests(
    mut menus: ResMut<ServerCellMenus>,
    mut requests: MessageReader<CellMenuOpenRequested>,
    mut opened: MessageWriter<CellMenuOpened>,
) {
    for request in requests.read() {
        match menus.open_or_create(
            request.player_id,
            request.menu_id.clone(),
            request.title.clone(),
            request.audience.clone(),
            request.layout.clone(),
        ) {
            Ok(menu) => {
                opened.write(CellMenuOpened {
                    viewer: request.player_id,
                    menu,
                });
            }
            Err(error) => debug!("ignored cell-menu open request: {error:?}"),
        };
    }
}

fn apply_move_requests(
    mut menus: ResMut<ServerCellMenus>,
    mut requests: MessageReader<CellMenuMoveRequested>,
    mut cells: MessageWriter<CellMenuCellSet>,
) {
    for request in requests.read() {
        let changes = match menus.move_or_swap(
            request.player_id,
            &request.menu_id,
            &request.from,
            &request.to,
        ) {
            Ok(Some(changes)) => changes,
            Ok(None) => continue,
            Err(error) => {
                debug!("ignored cell-menu move request: {error:?}");
                continue;
            }
        };
        for viewer in menus.viewers(&request.menu_id) {
            for (cell, item) in &changes {
                cells.write(CellMenuCellSet {
                    viewer,
                    menu_id: request.menu_id.clone(),
                    cell: cell.clone(),
                    item: item.clone(),
                });
            }
        }
    }
}

fn apply_close_requests(
    mut menus: ResMut<ServerCellMenus>,
    mut requests: MessageReader<CellMenuCloseRequested>,
    mut closed: MessageWriter<CellMenuClosed>,
) {
    for request in requests.read() {
        if menus.close(request.player_id, &request.menu_id) {
            closed.write(CellMenuClosed {
                viewer: request.player_id,
                menu_id: request.menu_id.clone(),
            });
        }
    }
}
