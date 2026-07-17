use bevy::prelude::*;

pub const DEFAULT_PLAYER_SPEED_MULTIPLIER: f32 = 1.0;

#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct PlayerSpeedMultiplier(pub f32);

impl Default for PlayerSpeedMultiplier {
    fn default() -> Self {
        Self(DEFAULT_PLAYER_SPEED_MULTIPLIER)
    }
}

pub trait PlayerSpeedApi: Send + Sync + 'static {}
