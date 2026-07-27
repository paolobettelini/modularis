use bevy::prelude::*;
use player_network_message_types::PlayerId;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use world_instance_api::WorldInstanceId;

#[derive(Resource, Clone, Default)]
pub struct ServerPlayerWorlds(Arc<RwLock<HashMap<PlayerId, WorldInstanceId>>>);

impl ServerPlayerWorlds {
    pub fn world(&self, player_id: PlayerId) -> Option<WorldInstanceId> {
        self.0
            .read()
            .expect("server player worlds lock poisoned")
            .get(&player_id)
            .cloned()
    }

    pub fn set(&self, player_id: PlayerId, world: WorldInstanceId) -> Option<WorldInstanceId> {
        self.0
            .write()
            .expect("server player worlds lock poisoned")
            .insert(player_id, world)
    }

    pub fn remove(&self, player_id: PlayerId) -> Option<WorldInstanceId> {
        self.0
            .write()
            .expect("server player worlds lock poisoned")
            .remove(&player_id)
    }
}

#[derive(Message, Debug, Clone)]
pub struct RequestServerPlayerWorldChange {
    pub player_id: PlayerId,
    pub world: WorldInstanceId,
    pub position: [f32; 3],
}

#[derive(Message, Debug, Clone)]
pub struct ServerPlayerWorldChanged {
    pub player_id: PlayerId,
    pub previous: Option<WorldInstanceId>,
    pub current: WorldInstanceId,
    pub position: [f32; 3],
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerPlayerWorldSet {
    Request,
    Apply,
    Sync,
}

pub trait ServerPlayerWorldApi: Send + Sync + 'static {}
