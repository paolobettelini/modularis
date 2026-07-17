use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_block_interaction_events_api::{
    ClientBlockInteractionSet, LocalBlockUseHandled, LocalBlockUseIntent,
};
use tokio::task::JoinHandle;

pub struct ClientBlockInteractionEventsMod;

impl ClientBlockInteractionEventsMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .add_message::<LocalBlockUseIntent>()
            .add_message::<LocalBlockUseHandled>()
            .configure_sets(
                Update,
                (
                    ClientBlockInteractionSet::Raycast,
                    ClientBlockInteractionSet::RoutingRules,
                    ClientBlockInteractionSet::SpecificHandlers,
                    ClientBlockInteractionSet::Fallback,
                )
                    .chain(),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
