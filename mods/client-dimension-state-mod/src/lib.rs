use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_dimension_api::{
    ClientDimension, ClientDimensionApi, ClientDimensionChanged, ClientDimensionSet,
};
use tokio::task::JoinHandle;

pub struct ClientDimensionStateMod;

impl ClientDimensionStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .init_resource::<ClientDimension>()
            .add_message::<ClientDimensionChanged>()
            .configure_sets(
                Update,
                (
                    ClientDimensionSet::Receive,
                    ClientDimensionSet::ResetWorld,
                    ClientDimensionSet::ApplyPlayer,
                )
                    .chain(),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientDimensionApi for ClientDimensionStateMod {}
