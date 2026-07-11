use bevy::prelude::*;
pub use generated_dimension_registry::Dimension;
use player_network_message_types::PlayerId;
use server_chunk_provider_api::ChunkProviderId;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use world_instance_api::WorldInstanceId;

#[derive(Debug, Clone, PartialEq)]
pub struct DimensionDefinition {
    pub id: Dimension,
    pub instance: WorldInstanceId,
    pub provider: ChunkProviderId,
    pub sky_color: [f32; 4],
    pub spawn: [f32; 3],
}

#[derive(Default)]
struct DimensionState {
    default: Option<Dimension>,
    definitions: HashMap<Dimension, DimensionDefinition>,
    players: HashMap<PlayerId, Dimension>,
}

#[derive(Resource, Clone, Default)]
pub struct ServerDimensions(Arc<RwLock<DimensionState>>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DimensionRegistrationError {
    DuplicateId(Dimension),
    DefaultAlreadyRegistered(Dimension),
}

impl ServerDimensions {
    pub fn register(
        &self,
        definition: DimensionDefinition,
        is_default: bool,
    ) -> Result<(), DimensionRegistrationError> {
        let mut state = self.0.write().expect("server dimensions lock poisoned");
        if state.definitions.contains_key(&definition.id) {
            return Err(DimensionRegistrationError::DuplicateId(definition.id));
        }
        if is_default {
            if let Some(default) = state.default {
                return Err(DimensionRegistrationError::DefaultAlreadyRegistered(
                    default,
                ));
            }
            state.default = Some(definition.id);
        }
        state.definitions.insert(definition.id, definition);
        Ok(())
    }

    pub fn default_dimension(&self) -> Option<DimensionDefinition> {
        let state = self.0.read().expect("server dimensions lock poisoned");
        let id = state.default.as_ref()?;
        state.definitions.get(id).cloned()
    }

    pub fn definition(&self, id: Dimension) -> Option<DimensionDefinition> {
        self.0
            .read()
            .expect("server dimensions lock poisoned")
            .definitions
            .get(&id)
            .cloned()
    }

    pub fn dimension_id_for(&self, player_id: PlayerId) -> Option<Dimension> {
        let state = self.0.read().expect("server dimensions lock poisoned");
        state
            .players
            .get(&player_id)
            .or(state.default.as_ref())
            .copied()
    }

    pub fn dimension_for(&self, player_id: PlayerId) -> Option<DimensionDefinition> {
        let state = self.0.read().expect("server dimensions lock poisoned");
        let id = state.players.get(&player_id).or(state.default.as_ref())?;
        state.definitions.get(id).cloned()
    }

    pub fn set_player(&self, player_id: PlayerId, dimension: Dimension) -> Option<Dimension> {
        let mut state = self.0.write().expect("server dimensions lock poisoned");
        if !state.definitions.contains_key(&dimension) {
            return None;
        }
        let previous = state.players.get(&player_id).copied().or(state.default)?;
        state.players.insert(player_id, dimension);
        Some(previous)
    }

    pub fn remove_player(&self, player_id: PlayerId) {
        self.0
            .write()
            .expect("server dimensions lock poisoned")
            .players
            .remove(&player_id);
    }
}

#[derive(Message, Debug, Clone)]
pub struct RequestPlayerDimensionChange {
    pub player_id: PlayerId,
    pub target: Dimension,
    pub position: Option<[f32; 3]>,
}

#[derive(Message, Debug, Clone)]
pub struct ServerPlayerDimensionChanged {
    pub player_id: PlayerId,
    pub previous: Dimension,
    pub current: DimensionDefinition,
    pub position: [f32; 3],
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerDimensionSet {
    Apply,
    Sync,
}

pub trait ServerDimensionApi: Send + Sync + 'static {}

#[cfg(test)]
mod tests {
    use super::*;

    fn definition(id: Dimension) -> DimensionDefinition {
        DimensionDefinition {
            id,
            instance: WorldInstanceId::new(generated_dimension_registry::id(id)),
            provider: ChunkProviderId::new(generated_dimension_registry::id(id)),
            sky_color: [0.0; 4],
            spawn: [0.0; 3],
        }
    }

    #[test]
    fn contributor_order_does_not_choose_the_default() {
        let dimensions = ServerDimensions::default();
        dimensions
            .register(definition(Dimension::Nether), false)
            .unwrap();
        assert!(dimensions.default_dimension().is_none());
        dimensions
            .register(definition(Dimension::Overworld), true)
            .unwrap();
        assert_eq!(
            dimensions.default_dimension().unwrap().id,
            Dimension::Overworld
        );
    }

    #[test]
    fn duplicate_ids_are_rejected() {
        let dimensions = ServerDimensions::default();
        dimensions
            .register(definition(Dimension::Overworld), true)
            .unwrap();
        assert_eq!(
            dimensions.register(definition(Dimension::Overworld), false),
            Err(DimensionRegistrationError::DuplicateId(
                Dimension::Overworld
            ))
        );
    }
}
