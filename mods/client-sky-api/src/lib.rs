use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy)]
pub struct ClientSkyColor(pub [f32; 4]);

impl Default for ClientSkyColor {
    fn default() -> Self {
        Self([0.48, 0.72, 0.94, 1.0])
    }
}

pub trait ClientSkyApi: Send + Sync + 'static {}
