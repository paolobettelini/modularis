use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_world_context_api::{
    ClientWorldChanged, ClientWorldContext, ClientWorldContextApi, ClientWorldContextSet,
};
use tokio::task::JoinHandle;

pub struct ClientWorldContextStateMod;

impl ClientWorldContextStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .init_resource::<ClientWorldContext>()
            .add_message::<ClientWorldChanged>()
            .configure_sets(
                Update,
                (
                    ClientWorldContextSet::Receive,
                    ClientWorldContextSet::ResetWorld,
                    ClientWorldContextSet::ApplyPlayer,
                )
                    .chain(),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientWorldContextApi for ClientWorldContextStateMod {}
