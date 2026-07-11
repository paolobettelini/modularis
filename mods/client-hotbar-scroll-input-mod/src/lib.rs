use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_mod::BevyMod;
use client_game_state_api::{GameStateApi, InGameOverlayState};
use client_inventory_cache_api::{ClientInventoryCache, ClientInventoryCacheApi};
use inventory_events_api::LocalHotbarSelectIntent;
use inventory_events_mod::InventoryEventsMod;
use tokio::task::JoinHandle;

pub struct ClientHotbarScrollInputMod;

impl ClientHotbarScrollInputMod {
    pub fn init<G: GameStateApi, C: ClientInventoryCacheApi>(
        bevy: &mut BevyMod,
        _game_state: &mut G,
        _cache: &mut C,
        _events: &mut InventoryEventsMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            cycle_with_mouse_wheel.run_if(in_state(InGameOverlayState::Playing)),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn cycle_with_mouse_wheel(
    mut wheel: MessageReader<MouseWheel>,
    cache: Res<ClientInventoryCache>,
    mut selections: MessageWriter<LocalHotbarSelectIntent>,
) {
    let scroll = wheel.read().map(|event| event.y).sum::<f32>();
    if scroll.abs() < f32::EPSILON {
        return;
    }

    let hotbar_size = cache
        .inventory
        .as_ref()
        .and_then(|inventory| inventory.layout.hotbar())
        .map_or(0, |hotbar| hotbar.cells);
    if hotbar_size == 0 {
        return;
    }

    let current = cache.selected_hotbar.min(hotbar_size.saturating_sub(1));
    let direction = if scroll > 0.0 { -1 } else { 1 };
    let next = (current as i32 + direction).rem_euclid(hotbar_size as i32) as u32;
    selections.write(LocalHotbarSelectIntent { index: next });
}
