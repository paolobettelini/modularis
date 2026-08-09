use bevy::{picking::hover::HoverMap, prelude::*};
use bevy_mod::BevyMod;
use client_game_state_api::{GameStateApi, InGameOverlayState};
use client_inventory_cache_api::{ClientInventoryCache, ClientInventoryCacheApi};
use client_inventory_ui_api::{
    ClientInventoryUiApi, InventoryItemVisual, InventoryOperationSequence, InventoryUiInputCapture,
};
use inventory_core_api::InventoryCell;
use inventory_events_api::LocalInventoryMoveIntent;
use inventory_events_mod::InventoryEventsMod;
use tokio::task::JoinHandle;

pub struct ClientInventoryNumberSwapMod;

impl ClientInventoryNumberSwapMod {
    pub fn init<U: ClientInventoryUiApi, C: ClientInventoryCacheApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _ui: &mut U,
        _cache: &mut C,
        _events: &mut InventoryEventsMod,
        _game_state: &mut G,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            number_swap.run_if(in_state(InGameOverlayState::Inventory)),
        );
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn number_swap(
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<InventoryUiInputCapture>,
    hover: Res<HoverMap>,
    items: Query<&InventoryItemVisual>,
    cache: Res<ClientInventoryCache>,
    mut sequence: ResMut<InventoryOperationSequence>,
    mut moves: MessageWriter<LocalInventoryMoveIntent>,
) {
    if capture.0 { return; }
    let Some(index) = pressed_number(&keyboard) else { return; };
    let Some(source) = hover.values().flat_map(|entities| entities.keys()).find_map(|entity| items.get(*entity).ok()) else { return; };
    let Some(inventory) = cache.inventory.as_ref() else { return; };
    let Some(hotbar) = inventory.layout.hotbar() else { return; };
    if index >= hotbar.cells { return; }
    let target = InventoryCell { section: hotbar.id.clone(), index };
    if source.cell == target { return; }
    moves.write(LocalInventoryMoveIntent {
        operation_id: sequence.next(),
        from: source.cell.clone(),
        to: target,
    });
}

fn pressed_number(keyboard: &ButtonInput<KeyCode>) -> Option<u32> {
    [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6, KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9]
        .into_iter()
        .position(|key| keyboard.just_pressed(key))
        .map(|index| index as u32)
}
