use bevy::prelude::*;

pub const DEFAULT_PLAYER_SCALE: f32 = 1.0;

#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct PlayerScale(pub f32);

impl Default for PlayerScale {
    fn default() -> Self {
        Self(DEFAULT_PLAYER_SCALE)
    }
}

pub trait PlayerScaleApi: Send + Sync + 'static {}
