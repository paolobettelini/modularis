use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct ClientWind {
    /// Normalized direction in the world XZ plane.
    pub direction: Vec2,
    /// Visual wind intensity. Zero means calm.
    pub intensity: f32,
}

impl Default for ClientWind {
    fn default() -> Self {
        Self {
            direction: Vec2::X,
            intensity: 1.0,
        }
    }
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct ClientWindChanged {
    pub previous: ClientWind,
    pub current: ClientWind,
}

pub trait ClientWindApi: Send + Sync + 'static {}
