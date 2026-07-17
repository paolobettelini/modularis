use bevy::prelude::*;
use player_flight_speed_api::DEFAULT_PLAYER_FLIGHT_SPEED_MULTIPLIER;
use player_network_message_types::PlayerId;
use std::collections::HashMap;

#[derive(Resource, Debug)]
pub struct ServerPlayerFlightSpeeds {
    default: f32,
    overrides: HashMap<PlayerId, f32>,
}

impl Default for ServerPlayerFlightSpeeds {
    fn default() -> Self {
        Self {
            default: DEFAULT_PLAYER_FLIGHT_SPEED_MULTIPLIER,
            overrides: HashMap::new(),
        }
    }
}

impl ServerPlayerFlightSpeeds {
    pub fn multiplier(&self, player_id: PlayerId) -> f32 {
        self.overrides
            .get(&player_id)
            .copied()
            .unwrap_or(self.default)
    }

    pub fn set(&mut self, player_id: PlayerId, multiplier: f32) -> bool {
        let previous = self.multiplier(player_id);
        if (multiplier - self.default).abs() <= f32::EPSILON {
            self.overrides.remove(&player_id);
        } else {
            self.overrides.insert(player_id, multiplier);
        }
        (previous - multiplier).abs() > f32::EPSILON
    }

    pub fn remove(&mut self, player_id: PlayerId) {
        self.overrides.remove(&player_id);
    }
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct SetServerPlayerFlightSpeed {
    pub player_id: PlayerId,
    pub multiplier: f32,
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct ServerPlayerFlightSpeedChanged {
    pub player_id: PlayerId,
    pub multiplier: f32,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerPlayerFlightSpeedSet {
    Apply,
    Sync,
}

pub trait ServerPlayerFlightSpeedApi: Send + Sync + 'static {}
