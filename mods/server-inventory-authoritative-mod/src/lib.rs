use bevy::prelude::*;
use bevy_mod::BevyMod;
use inventory_events_api::*;
use inventory_events_mod::InventoryEventsMod;
use server_inventory_api::{ServerInventories, ServerInventoryApi};
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use std::collections::HashSet;
use tokio::task::JoinHandle;

pub struct ServerInventoryAuthoritativeMod;

impl ServerInventoryAuthoritativeMod {
    pub fn init(
        bevy: &mut BevyMod,
        _events: &mut InventoryEventsMod,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
    ) -> Self {
        bevy.app
            .init_resource::<ServerInventories>()
            .add_systems(
                Update,
                move_or_swap.in_set(InventoryValidationSet::MoveOrSwap),
            )
            .add_systems(
                Update,
                (
                    apply_resets,
                    apply_resizes,
                    apply_cell_sets,
                    apply_hotbar_selection,
                    answer_sync_requests,
                    remove_left_players,
                )
                    .chain()
                    .in_set(InventoryValidationSet::Other),
            )
            .add_systems(
                Update,
                dispatch_held_item_use.in_set(InventoryServerSet::DispatchUse),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerInventoryApi for ServerInventoryAuthoritativeMod {}

fn move_or_swap(
    mut inventories: ResMut<ServerInventories>,
    mut requests: MessageReader<InventoryMoveRequested>,
    mut handled: MessageReader<InventoryMoveHandled>,
    mut changed: MessageWriter<InventoryCellSet>,
) {
    let handled = handled
        .read()
        .map(|event| (event.player_id, event.operation_id))
        .collect::<HashSet<_>>();
    for request in requests.read() {
        if handled.contains(&(request.player_id, request.operation_id)) {
            continue;
        }
        let Some(state) = inventories.get_mut(request.player_id) else {
            continue;
        };
        if state
            .inventory
            .move_or_swap(&request.from, &request.to)
            .unwrap_or(false)
        {
            changed.write(InventoryCellSet {
                player_id: request.player_id,
                cell: request.from.clone(),
                item: state.inventory.get(&request.from).cloned(),
            });
            changed.write(InventoryCellSet {
                player_id: request.player_id,
                cell: request.to.clone(),
                item: state.inventory.get(&request.to).cloned(),
            });
        }
    }
}

fn apply_resets(
    mut inventories: ResMut<ServerInventories>,
    mut requests: MessageReader<InventoryResetRequested>,
    mut applied: MessageWriter<InventoryResetApplied>,
) {
    for request in requests.read() {
        let selected_hotbar = request.inventory.layout.hotbar().map_or(0, |hotbar| {
            request.selected_hotbar.min(hotbar.cells.saturating_sub(1))
        });
        inventories.reset(
            request.player_id,
            request.inventory.clone(),
            selected_hotbar,
        );
        applied.write(InventoryResetApplied {
            player_id: request.player_id,
            inventory: request.inventory.clone(),
            selected_hotbar,
        });
    }
}

fn apply_resizes(
    mut inventories: ResMut<ServerInventories>,
    mut requests: MessageReader<InventoryResizeRequested>,
    mut applied: MessageWriter<InventoryResized>,
    mut selections: MessageWriter<HotbarSelectionSet>,
) {
    for request in requests.read() {
        if inventories
            .resize(request.player_id, request.layout.clone())
            .is_ok()
        {
            applied.write(InventoryResized {
                player_id: request.player_id,
                layout: request.layout.clone(),
            });
            let Some(state) = inventories.get_mut(request.player_id) else {
                continue;
            };
            let selected = state.inventory.layout.hotbar().map_or(0, |hotbar| {
                state.selected_hotbar.min(hotbar.cells.saturating_sub(1))
            });
            if selected != state.selected_hotbar {
                state.selected_hotbar = selected;
                selections.write(HotbarSelectionSet {
                    player_id: request.player_id,
                    index: selected,
                });
            }
        }
    }
}

fn apply_cell_sets(
    mut inventories: ResMut<ServerInventories>,
    mut requests: MessageReader<InventorySetCellRequested>,
    mut applied: MessageWriter<InventoryCellSet>,
) {
    for request in requests.read() {
        if inventories
            .set_cell(
                request.player_id,
                request.cell.clone(),
                request.item.clone(),
            )
            .is_ok()
        {
            applied.write(InventoryCellSet {
                player_id: request.player_id,
                cell: request.cell.clone(),
                item: request.item.clone(),
            });
        }
    }
}

fn apply_hotbar_selection(
    mut inventories: ResMut<ServerInventories>,
    mut requests: MessageReader<HotbarSelectRequested>,
    mut applied: MessageWriter<HotbarSelectionSet>,
) {
    for request in requests.read() {
        let Some(state) = inventories.get_mut(request.player_id) else {
            continue;
        };
        let Some(hotbar) = state.inventory.layout.hotbar() else {
            continue;
        };
        if request.index < hotbar.cells && request.index != state.selected_hotbar {
            state.selected_hotbar = request.index;
            applied.write(HotbarSelectionSet {
                player_id: request.player_id,
                index: request.index,
            });
        }
    }
}

fn dispatch_held_item_use(
    inventories: Res<ServerInventories>,
    mut requests: MessageReader<UseHeldItemRequested>,
    mut dispatched: MessageWriter<HeldItemUseDispatched>,
) {
    for request in requests.read() {
        let Some(state) = inventories.get(request.player_id) else {
            continue;
        };
        let Some(hotbar) = state.inventory.layout.hotbar() else {
            continue;
        };
        let cell = inventory_core_api::InventoryCell {
            section: hotbar.id.clone(),
            index: state.selected_hotbar,
        };
        let Some(item) = state.inventory.get(&cell).cloned() else {
            continue;
        };
        dispatched.write(HeldItemUseDispatched {
            player_id: request.player_id,
            cell,
            item,
            target: request.target.clone(),
        });
    }
}

fn answer_sync_requests(
    inventories: Res<ServerInventories>,
    mut requests: MessageReader<InventorySyncRequested>,
    mut resets: MessageWriter<InventoryResetApplied>,
) {
    for request in requests.read() {
        let Some(state) = inventories.get(request.player_id) else {
            continue;
        };
        resets.write(InventoryResetApplied {
            player_id: request.player_id,
            inventory: state.inventory.clone(),
            selected_hotbar: state.selected_hotbar,
        });
    }
}

fn remove_left_players(
    mut inventories: ResMut<ServerInventories>,
    mut left: MessageReader<ServerPlayerLeft>,
) {
    for event in left.read() {
        inventories.remove(event.player_id);
    }
}
