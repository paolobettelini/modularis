use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy)]
pub struct JumpConfig {
    pub speed: f32,
    pub rearm_seconds: f32,
}

impl Default for JumpConfig {
    fn default() -> Self {
        Self {
            speed: 7.5,
            rearm_seconds: 0.18,
        }
    }
}

pub trait PlayerJumpApi: Send + Sync + 'static {}
