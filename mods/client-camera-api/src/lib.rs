use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct PlayerCamera;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct CameraAngles {
    pub yaw: f32,
    pub pitch: f32,
}

pub trait CameraApi: Send + Sync + 'static {}
