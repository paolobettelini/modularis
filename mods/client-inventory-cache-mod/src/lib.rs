use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_inventory_cache_api::{ClientInventoryCache, ClientInventoryCacheApi};
use inventory_events_api::{
    ClientHotbarSelectionSet, ClientInventoryCellSet, ClientInventoryReset, ClientInventoryResized,
    InventoryClientCacheSet,
};
use inventory_events_mod::InventoryEventsMod;
use tokio::task::JoinHandle;

pub struct ClientInventoryCacheMod;

impl ClientInventoryCacheMod {
    pub fn init(bevy: &mut BevyMod, _events: &mut InventoryEventsMod) -> Self {
        bevy.app
            .init_resource::<ClientInventoryCache>()
            .add_systems(
                Update,
                apply_inventory_sync.in_set(InventoryClientCacheSet::AuthoritativeSync),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientInventoryCacheApi for ClientInventoryCacheMod {}

fn apply_inventory_sync(
    mut cache: ResMut<ClientInventoryCache>,
    mut resets: MessageReader<ClientInventoryReset>,
    mut resizes: MessageReader<ClientInventoryResized>,
    mut cells: MessageReader<ClientInventoryCellSet>,
    mut selections: MessageReader<ClientHotbarSelectionSet>,
) {
    for event in resets.read() {
        cache.inventory = Some(event.inventory.clone());
        cache.selected_hotbar = event.selected_hotbar;
        cache.content_revision += 1;
        cache.selection_revision += 1;
    }
    for event in resizes.read() {
        if let Some(inventory) = cache.inventory.as_mut()
            && inventory.resize(event.layout.clone()).is_ok()
        {
            cache.content_revision += 1;
        }
    }
    for event in cells.read() {
        if let Some(inventory) = cache.inventory.as_mut()
            && inventory
                .set(event.cell.clone(), event.item.clone())
                .is_ok()
        {
            cache.content_revision += 1;
        }
    }
    for event in selections.read() {
        cache.selected_hotbar = event.index;
        cache.selection_revision += 1;
    }
}
