use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_block_interaction_events_api::{
    ClientBlockInteractionSet, LocalBlockUseHandled, LocalBlockUseIntent,
};
use client_block_interaction_events_mod::ClientBlockInteractionEventsMod;
use inventory_events_api::LocalUseHeldItemIntent;
use inventory_events_mod::InventoryEventsMod;
use std::collections::HashSet;
use tokio::task::JoinHandle;

pub struct ClientUseHeldItemOnBlockMod;

impl ClientUseHeldItemOnBlockMod {
    pub fn init(
        bevy: &mut BevyMod,
        _interaction_events: &mut ClientBlockInteractionEventsMod,
        _inventory_events: &mut InventoryEventsMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            fallback_to_held_item_use.in_set(ClientBlockInteractionSet::Fallback),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn fallback_to_held_item_use(
    mut intents: MessageReader<LocalBlockUseIntent>,
    mut handled: MessageReader<LocalBlockUseHandled>,
    mut uses: MessageWriter<LocalUseHeldItemIntent>,
) {
    let handled = handled
        .read()
        .map(|event| event.operation_id)
        .collect::<HashSet<_>>();
    for intent in intents.read() {
        if handled.contains(&intent.operation_id) {
            continue;
        }
        uses.write(LocalUseHeldItemIntent {
            target: intent.target.clone(),
        });
    }
}
