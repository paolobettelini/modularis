use bevy::prelude::*;
use bevy_mod::BevyMod;
use inventory_events_api::{InventoryCellSet, InventoryServerSet, ItemUseSucceeded};
use inventory_events_mod::InventoryEventsMod;
use inventory_quantity_operations_lib::{QuantityConsumption, consume_one};
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
        let Some(current) = state.inventory.get(&item_use.cell).cloned() else {
            continue;
        };
        let next = match consume_one(&current, &item_use.item_before_use) {
            QuantityConsumption::NotApplicable | QuantityConsumption::Unchanged => continue,
            QuantityConsumption::Remove => None,
            QuantityConsumption::Replace(next) => Some(next),
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
