use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_block_interaction_events_api::{
    ClientBlockInteractionSet, LocalBlockUseHandled, LocalBlockUseIntent,
};
use client_block_interaction_events_mod::ClientBlockInteractionEventsMod;
use inventory_events_api::LocalUseHeldItemIntent;
use inventory_events_mod::InventoryEventsMod;
use player_sneak_api::{LocalPlayerSneak, PlayerSneakApi, PlayerSneakSet};
use tokio::task::JoinHandle;

pub struct ClientPlayerSneakBlockInteractionBypassVanillaMod;

impl ClientPlayerSneakBlockInteractionBypassVanillaMod {
    pub fn init<S: PlayerSneakApi>(
        bevy: &mut BevyMod,
        _interaction_events: &mut ClientBlockInteractionEventsMod,
        _inventory_events: &mut InventoryEventsMod,
        _sneak: &mut S,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            route_sneaking_use_to_held_item
                .in_set(ClientBlockInteractionSet::RoutingRules)
                .after(PlayerSneakSet::Input),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn route_sneaking_use_to_held_item(
    sneak: Res<LocalPlayerSneak>,
    mut intents: MessageReader<LocalBlockUseIntent>,
    mut handled: MessageWriter<LocalBlockUseHandled>,
    mut uses: MessageWriter<LocalUseHeldItemIntent>,
) {
    if !sneak.active {
        return;
    }
    for intent in intents.read() {
        uses.write(LocalUseHeldItemIntent {
            target: intent.target.clone(),
        });
        handled.write(LocalBlockUseHandled {
            operation_id: intent.operation_id,
        });
    }
}
