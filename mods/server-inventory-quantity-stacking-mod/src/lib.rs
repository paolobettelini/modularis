use bevy::prelude::*;
use bevy_mod::BevyMod;
use inventory_events_api::{
    InventoryCellSet, InventoryMoveHandled, InventoryMoveRequested, InventoryValidationSet,
};
use inventory_events_mod::InventoryEventsMod;
use item_quantity_meta::Quantity;
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
        let Some(mut source) = state.inventory.get(&request.from).cloned() else {
            continue;
        };
        let Some(mut target) = state.inventory.get(&request.to).cloned() else {
            continue;
        };
        if source.item != target.item {
            continue;
        }
        let (Some(source_quantity), Some(target_quantity)) =
            (source.metadata.quantity, target.metadata.quantity)
        else {
            continue;
        };
        source.metadata.quantity = None;
        target.metadata.quantity = None;
        if source.metadata != target.metadata {
            continue;
        }
        target.metadata.quantity = Some(merge(source_quantity, target_quantity));
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

fn merge(left: Quantity, right: Quantity) -> Quantity {
    match (left, right) {
        (Quantity::Infinite, _) | (_, Quantity::Infinite) => Quantity::Infinite,
        (Quantity::Finite(left), Quantity::Finite(right)) => {
            Quantity::Finite(left.saturating_add(right))
        }
    }
}
