use bevy::prelude::*;
use player_network_message_types::PlayerId;
use std::collections::HashMap;

#[derive(Resource, Debug)]
pub struct ServerPlayerGravities {
    default: Vec3,
    overrides: HashMap<PlayerId, Vec3>,
}

impl ServerPlayerGravities {
    pub fn new(default: Vec3) -> Self {
        Self {
            default,
            overrides: HashMap::new(),
        }
    }

    pub fn gravity(&self, player_id: PlayerId) -> Vec3 {
        self.overrides
            .get(&player_id)
            .copied()
            .unwrap_or(self.default)
    }

    pub fn set(&mut self, player_id: PlayerId, gravity: Vec3) -> bool {
        let previous = self.gravity(player_id);
        if gravity.abs_diff_eq(self.default, f32::EPSILON) {
            self.overrides.remove(&player_id);
        } else {
            self.overrides.insert(player_id, gravity);
        }
        !previous.abs_diff_eq(gravity, f32::EPSILON)
    }

    pub fn remove(&mut self, player_id: PlayerId) {
        self.overrides.remove(&player_id);
    }
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct SetServerPlayerGravity {
    pub player_id: PlayerId,
    pub gravity: Vec3,
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct ServerPlayerGravityChanged {
    pub player_id: PlayerId,
    pub gravity: Vec3,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerPlayerGravitySet {
    Apply,
    Sync,
}

pub trait ServerPlayerGravityApi: Send + Sync + 'static {}
