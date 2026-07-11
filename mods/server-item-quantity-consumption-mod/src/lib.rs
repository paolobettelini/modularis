use bevy::prelude::*;
use bevy_mod::BevyMod;
use inventory_events_api::{InventoryCellSet, InventoryServerSet, ItemUseSucceeded};
use inventory_events_mod::InventoryEventsMod;
use item_quantity_meta::Quantity;
use server_inventory_api::{ServerInventories, ServerInventoryApi};
use tokio::task::JoinHandle;

pub struct ServerItemQuantityConsumptionMod;

impl ServerItemQuantityConsumptionMod {
    pub fn init<I: ServerInventoryApi>(
        bevy: &mut BevyMod,
        _events: &mut InventoryEventsMod,
        _inventories: &mut I,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            consume_successful_uses.in_set(InventoryServerSet::ApplyConsumption),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn consume_successful_uses(
    mut inventories: ResMut<ServerInventories>,
    mut uses: MessageReader<ItemUseSucceeded>,
    mut changed: MessageWriter<InventoryCellSet>,
) {
    for item_use in uses.read() {
        let Some(state) = inventories.get_mut(item_use.player_id) else {
            continue;
        };
        let Some(mut current) = state.inventory.get(&item_use.cell).cloned() else {
            continue;
        };
        if current.item != item_use.item_before_use.item {
            continue;
        }
        let Some(quantity) = current.metadata.quantity else {
            continue;
        };
        let next = match quantity {
            Quantity::Infinite => continue,
            Quantity::Finite(0 | 1) => None,
            Quantity::Finite(value) => {
                current.metadata.quantity = Some(Quantity::Finite(value - 1));
                Some(current)
            }
        };
        if state
            .inventory
            .set(item_use.cell.clone(), next.clone())
            .is_ok()
        {
            changed.write(InventoryCellSet {
                player_id: item_use.player_id,
                cell: item_use.cell.clone(),
                item: next,
            });
        }
    }
}
