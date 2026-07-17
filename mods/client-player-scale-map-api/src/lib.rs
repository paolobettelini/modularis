use bevy::prelude::*;
use player_network_message_types::PlayerId;
use player_scale_api::DEFAULT_PLAYER_SCALE;
use std::collections::HashMap;

#[derive(Resource, Debug, Default)]
pub struct ClientPlayerScales {
    values: HashMap<PlayerId, f32>,
}

impl ClientPlayerScales {
    pub fn scale(&self, player_id: PlayerId) -> f32 {
        self.values
            .get(&player_id)
            .copied()
            .unwrap_or(DEFAULT_PLAYER_SCALE)
    }

    pub fn set(&mut self, player_id: PlayerId, scale: f32) -> bool {
        let previous = self.scale(player_id);
        if (scale - DEFAULT_PLAYER_SCALE).abs() <= f32::EPSILON {
            self.values.remove(&player_id);
        } else {
            self.values.insert(player_id, scale);
        }
        (previous - scale).abs() > f32::EPSILON
    }

    pub fn remove(&mut self, player_id: PlayerId) {
        self.values.remove(&player_id);
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct ClientPlayerScaleChanged {
    pub player_id: PlayerId,
    pub scale: f32,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientPlayerScaleMapSet;

pub trait ClientPlayerScaleMapApi: Send + Sync + 'static {}
