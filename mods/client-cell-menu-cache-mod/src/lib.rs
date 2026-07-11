use bevy::prelude::*;
use bevy_mod::BevyMod;
use cell_menu_api::{
    CellMenuClientSet, ClientCellMenuCellSet, ClientCellMenuClosed, ClientCellMenuOpened,
};
use cell_menu_events_mod::CellMenuEventsMod;
use client_cell_menu_cache_api::{ClientCellMenuCache, ClientCellMenuCacheApi};
use tokio::task::JoinHandle;

pub struct ClientCellMenuCacheMod;

impl ClientCellMenuCacheMod {
    pub fn init(bevy: &mut BevyMod, _events: &mut CellMenuEventsMod) -> Self {
        bevy.app.init_resource::<ClientCellMenuCache>().add_systems(
            Update,
            apply_cell_menu_sync.in_set(CellMenuClientSet::ApplyCache),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientCellMenuCacheApi for ClientCellMenuCacheMod {}

fn apply_cell_menu_sync(
    mut cache: ResMut<ClientCellMenuCache>,
    mut opened: MessageReader<ClientCellMenuOpened>,
    mut closed: MessageReader<ClientCellMenuClosed>,
    mut cells: MessageReader<ClientCellMenuCellSet>,
) {
    for event in opened.read() {
        cache.active = Some(event.menu.id.clone());
        cache
            .menus
            .insert(event.menu.id.clone(), event.menu.clone());
        cache.content_revision += 1;
    }
    for event in cells.read() {
        if let Some(menu) = cache.menus.get_mut(&event.menu_id)
            && menu
                .inventory
                .set(event.cell.clone(), event.item.clone())
                .is_ok()
        {
            cache.content_revision += 1;
        }
    }
    for event in closed.read() {
        cache.menus.remove(&event.menu_id);
        if cache.active.as_ref() == Some(&event.menu_id) {
            cache.active = None;
        }
        cache.content_revision += 1;
    }
}
