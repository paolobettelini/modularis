use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy)]
pub struct JumpConfig {
    pub speed: f32,
    pub rearm_seconds: f32,
    /// How long a press remains pending while waiting for a physics tick or
    /// ground contact.
    pub input_buffer_seconds: f32,
}

impl Default for JumpConfig {
    fn default() -> Self {
        Self {
            speed: 0.42 * 20.0,
            rearm_seconds: 0.18,
            input_buffer_seconds: 0.12,
        }
    }
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalPlayerJumped;

pub trait PlayerJumpApi: Send + Sync + 'static {}
