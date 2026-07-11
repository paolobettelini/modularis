use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_dimension_api::{
    RequestPlayerDimensionChange, ServerDimensionApi, ServerDimensionSet, ServerDimensions,
    ServerPlayerDimensionChanged,
};
use server_player_lifecycle_events_api::{ServerPlayerJoined, ServerPlayerLeft};
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
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
                .in_set(ServerDimensionSet::Apply),
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
        let Some(definition) = dimensions.default_dimension() else {
            continue;
        };
        let previous = dimensions
            .set_player(event.player_id, definition.id)
            .unwrap_or(definition.id);
        registry.set_player_position(event.player_id, definition.spawn);
        changed.write(ServerPlayerDimensionChanged {
            player_id: event.player_id,
            previous,
            current: definition.clone(),
            position: definition.spawn,
        });
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
        let Some(definition) = dimensions.definition(request.target) else {
            continue;
        };
        let Some(previous) = dimensions.set_player(request.player_id, request.target) else {
            continue;
        };
        let position = request.position.unwrap_or(definition.spawn);
        if registry
            .set_player_position(request.player_id, position)
            .is_none()
        {
            continue;
        }
        changed.write(ServerPlayerDimensionChanged {
            player_id: request.player_id,
            previous,
            current: definition,
            position,
        });
    }
}
