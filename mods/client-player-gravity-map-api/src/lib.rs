use bevy::prelude::*;
use player_gravity_api::{DEFAULT_GRAVITY, Gravity};
use player_network_message_types::PlayerId;
use std::collections::HashMap;

#[derive(Resource, Debug, Default)]
pub struct ClientPlayerGravities {
    values: HashMap<PlayerId, Vec3>,
}

impl ClientPlayerGravities {
    pub fn gravity(&self, player_id: PlayerId) -> Gravity {
        Gravity(
            self.values
                .get(&player_id)
                .copied()
                .unwrap_or(DEFAULT_GRAVITY),
        )
    }

    pub fn set(&mut self, player_id: PlayerId, gravity: Vec3) -> bool {
        let previous = self.gravity(player_id).0;
        if gravity.abs_diff_eq(DEFAULT_GRAVITY, f32::EPSILON) {
            self.values.remove(&player_id);
        } else {
            self.values.insert(player_id, gravity);
        }
        !previous.abs_diff_eq(gravity, f32::EPSILON)
    }

    pub fn remove(&mut self, player_id: PlayerId) {
        self.values.remove(&player_id);
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct ClientPlayerGravityChanged {
    pub player_id: PlayerId,
    pub gravity: Vec3,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientPlayerGravityMapSet;

pub trait ClientPlayerGravityMapApi: Send + Sync + 'static {}
