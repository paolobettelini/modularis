use bevy::prelude::*;
use bevy_mod::BevyMod;
use inventory_events_api::{
    InventoryCellSet, InventoryMoveHandled, InventoryMoveRequested, InventoryValidationSet,
};
use inventory_events_mod::InventoryEventsMod;
use inventory_quantity_operations_lib::merge_compatible_items;
use server_inventory_api::{ServerInventories, ServerInventoryApi};
use tokio::task::JoinHandle;

pub struct ServerInventoryQuantityStackingMod;

impl ServerInventoryQuantityStackingMod {
    pub fn init<I: ServerInventoryApi>(
        bevy: &mut BevyMod,
        _events: &mut InventoryEventsMod,
        _inventories: &mut I,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            stack_compatible_items.in_set(InventoryValidationSet::Stack),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn stack_compatible_items(
    mut inventories: ResMut<ServerInventories>,
    mut requests: MessageReader<InventoryMoveRequested>,
    mut handled: MessageWriter<InventoryMoveHandled>,
    mut changed: MessageWriter<InventoryCellSet>,
) {
    for request in requests.read() {
        let Some(state) = inventories.get_mut(request.player_id) else {
            continue;
        };
        let Some(source) = state.inventory.get(&request.from).cloned() else {
            continue;
        };
        let Some(target) = state.inventory.get(&request.to).cloned() else {
            continue;
        };
        let Some(target) = merge_compatible_items(&source, &target) else {
            continue;
        };
        let _ = state.inventory.set(request.from.clone(), None);
        let _ = state
            .inventory
            .set(request.to.clone(), Some(target.clone()));
        handled.write(InventoryMoveHandled {
            operation_id: request.operation_id,
            player_id: request.player_id,
        });
        changed.write(InventoryCellSet {
            player_id: request.player_id,
            cell: request.from.clone(),
            item: None,
        });
        changed.write(InventoryCellSet {
            player_id: request.player_id,
            cell: request.to.clone(),
            item: Some(target),
        });
    }
}
