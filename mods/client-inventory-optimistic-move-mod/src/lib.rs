use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_inventory_cache_api::{ClientInventoryCache, ClientInventoryCacheApi};
use inventory_events_api::{InventoryClientCacheSet, LocalInventoryMoveIntent};
use inventory_events_mod::InventoryEventsMod;
use tokio::task::JoinHandle;

pub struct ClientInventoryOptimisticMoveMod;

impl ClientInventoryOptimisticMoveMod {
    pub fn init<C: ClientInventoryCacheApi>(
        bevy: &mut BevyMod,
        _cache: &mut C,
        _events: &mut InventoryEventsMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            apply_local_move_preview.in_set(InventoryClientCacheSet::OptimisticPreview),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_local_move_preview(
    mut cache: ResMut<ClientInventoryCache>,
    mut moves: MessageReader<LocalInventoryMoveIntent>,
) {
    for event in moves.read() {
        let Some(inventory) = cache.inventory.as_mut() else {
            continue;
        };
        if inventory
            .move_or_swap(&event.from, &event.to)
            .unwrap_or(false)
        {
            cache.content_revision += 1;
        }
    }
}
