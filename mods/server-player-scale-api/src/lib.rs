use bevy::prelude::*;
use player_network_message_types::PlayerId;
use player_scale_api::DEFAULT_PLAYER_SCALE;
use std::collections::HashMap;

#[derive(Resource, Debug)]
pub struct ServerPlayerScales {
    default: f32,
    overrides: HashMap<PlayerId, f32>,
}

impl Default for ServerPlayerScales {
    fn default() -> Self {
        Self {
            default: DEFAULT_PLAYER_SCALE,
            overrides: HashMap::new(),
        }
    }
}

impl ServerPlayerScales {
    pub fn scale(&self, player_id: PlayerId) -> f32 {
        self.overrides
            .get(&player_id)
            .copied()
            .unwrap_or(self.default)
    }

    pub fn set(&mut self, player_id: PlayerId, scale: f32) -> bool {
        let previous = self.scale(player_id);
        if (scale - self.default).abs() <= f32::EPSILON {
            self.overrides.remove(&player_id);
        } else {
            self.overrides.insert(player_id, scale);
        }
        (previous - scale).abs() > f32::EPSILON
    }

    pub fn remove(&mut self, player_id: PlayerId) {
        self.overrides.remove(&player_id);
    }
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct SetServerPlayerScale {
    pub player_id: PlayerId,
    pub scale: f32,
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct ServerPlayerScaleChanged {
    pub player_id: PlayerId,
    pub scale: f32,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerPlayerScaleSet {
    Apply,
    Sync,
}

pub trait ServerPlayerScaleApi: Send + Sync + 'static {}
