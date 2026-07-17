use bevy::prelude::*;

pub const DEFAULT_PLAYER_FLIGHT_SPEED_MULTIPLIER: f32 = 2.0;

#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct PlayerFlightSpeedMultiplier(pub f32);

impl Default for PlayerFlightSpeedMultiplier {
    fn default() -> Self {
        Self(DEFAULT_PLAYER_FLIGHT_SPEED_MULTIPLIER)
    }
}

pub trait PlayerFlightSpeedApi: Send + Sync + 'static {}
