use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_dimension_api::{
    RequestPlayerDimensionChange, ServerDimensionApi, ServerDimensionSet, ServerDimensions,
    ServerPlayerDimensionChanged,
};
use tokio::task::JoinHandle;

pub struct ServerDimensionRegistryMod;

impl ServerDimensionRegistryMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .init_resource::<ServerDimensions>()
            .add_message::<RequestPlayerDimensionChange>()
            .add_message::<ServerPlayerDimensionChanged>()
            .configure_sets(
                Update,
                (ServerDimensionSet::Apply, ServerDimensionSet::Sync).chain(),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerDimensionApi for ServerDimensionRegistryMod {}
