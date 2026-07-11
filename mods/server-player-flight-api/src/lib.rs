use bevy::prelude::*;
use player_network_message_types::PlayerId;
use std::collections::HashSet;

#[derive(Resource, Default)]
pub struct ServerPlayerFlightCapabilities(HashSet<PlayerId>);

impl ServerPlayerFlightCapabilities {
    pub fn enabled(&self, player_id: PlayerId) -> bool {
        self.0.contains(&player_id)
    }

    pub fn set(&mut self, player_id: PlayerId, enabled: bool) -> bool {
        if enabled {
            self.0.insert(player_id)
        } else {
            self.0.remove(&player_id)
        }
    }

    pub fn remove(&mut self, player_id: PlayerId) {
        self.0.remove(&player_id);
    }
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetPlayerFlightCapability {
    pub player_id: PlayerId,
    pub enabled: bool,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerPlayerFlightCapabilityChanged {
    pub player_id: PlayerId,
    pub enabled: bool,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerPlayerFlightSet {
    Apply,
    Sync,
}

pub trait ServerPlayerFlightApi: Send + Sync + 'static {}
