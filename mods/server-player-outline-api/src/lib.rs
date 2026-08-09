use bevy::prelude::*;
use player_network_message_types::PlayerId;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct ServerPlayerOutlines(HashMap<PlayerId, bool>);

impl ServerPlayerOutlines {
    pub fn enabled(&self, player_id: PlayerId) -> bool {
        self.0.get(&player_id).copied().unwrap_or(true)
    }

    pub fn set(&mut self, player_id: PlayerId, enabled: bool) -> bool {
        if self.enabled(player_id) == enabled { return false; }
        self.0.insert(player_id, enabled);
        true
    }

    pub fn remove(&mut self, player_id: PlayerId) { self.0.remove(&player_id); }
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetPlayerOutline {
    pub player_id: PlayerId,
    pub enabled: bool,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerPlayerOutlineChanged {
    pub player_id: PlayerId,
    pub enabled: bool,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerPlayerOutlineSet { Apply, Sync }

pub trait ServerPlayerOutlineApi: Send + Sync + 'static {}
