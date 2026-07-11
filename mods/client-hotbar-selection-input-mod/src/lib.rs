use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameStateApi, InGameOverlayState};
use client_inventory_cache_api::{ClientInventoryCache, ClientInventoryCacheApi};
use inventory_events_api::LocalHotbarSelectIntent;
use inventory_events_mod::InventoryEventsMod;
use tokio::task::JoinHandle;

pub struct ClientHotbarSelectionInputMod;

impl ClientHotbarSelectionInputMod {
    pub fn init<G: GameStateApi, C: ClientInventoryCacheApi>(
        bevy: &mut BevyMod,
        _game_state: &mut G,
        _cache: &mut C,
        _events: &mut InventoryEventsMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            select_with_number_keys.run_if(in_state(InGameOverlayState::Playing)),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn select_with_number_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    cache: Res<ClientInventoryCache>,
    mut selections: MessageWriter<LocalHotbarSelectIntent>,
) {
    let keys = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ];
    let hotbar_size = cache
        .inventory
        .as_ref()
        .and_then(|inventory| inventory.layout.hotbar())
        .map_or(0, |hotbar| hotbar.cells);
    for (index, key) in keys.into_iter().enumerate() {
        if index as u32 >= hotbar_size {
            break;
        }
        if keyboard.just_pressed(key) {
            selections.write(LocalHotbarSelectIntent {
                index: index as u32,
            });
            break;
        }
    }
}
