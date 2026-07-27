use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_dimension_api::{
    RequestPlayerDimensionChange, ServerDimensionApi, ServerDimensionSet, ServerDimensions,
    ServerPlayerDimensionChanged,
};
use server_player_dimension_lifecycle_lib::{apply_dimension_change, initialize_default_dimension};
use server_player_lifecycle_events_api::{ServerPlayerJoined, ServerPlayerLeft};
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::{
    ServerPlayerRegistry, ServerPlayerRegistryApi, ServerPlayerSessionSet,
};
use tokio::task::JoinHandle;

pub struct ServerPlayerDimensionLifecycleVanillaMod;

impl ServerPlayerDimensionLifecycleVanillaMod {
    pub fn init<D: ServerDimensionApi, P: ServerPlayerRegistryApi>(
        bevy: &mut BevyMod,
        _dimensions_api: &mut D,
        _players: &mut P,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            (initialize_joined_dimensions, apply_dimension_changes)
                .chain()
                .in_set(ServerDimensionSet::Apply)
                .in_set(ServerPlayerSessionSet::Initialize),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn initialize_joined_dimensions(
    dimensions: Res<ServerDimensions>,
    mut registry: ResMut<ServerPlayerRegistry>,
    mut joined: MessageReader<ServerPlayerJoined>,
    mut left: MessageReader<ServerPlayerLeft>,
    mut changed: MessageWriter<ServerPlayerDimensionChanged>,
) {
    for event in joined.read() {
        if let Some(change) =
            initialize_default_dimension(&dimensions, &mut registry, event.player_id)
        {
            changed.write(change);
        }
    }
    for event in left.read() {
        dimensions.remove_player(event.player_id);
    }
}

fn apply_dimension_changes(
    dimensions: Res<ServerDimensions>,
    mut registry: ResMut<ServerPlayerRegistry>,
    mut requests: MessageReader<RequestPlayerDimensionChange>,
    mut changed: MessageWriter<ServerPlayerDimensionChanged>,
) {
    for request in requests.read() {
        if let Some(change) = apply_dimension_change(&dimensions, &mut registry, request) {
            changed.write(change);
        }
    }
}
