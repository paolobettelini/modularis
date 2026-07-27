use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct ClientGrassSettings {
    pub enabled: bool,
    pub blades_per_block: u32,
    pub sparsity: f32,
    pub blade_height: f32,
    pub height_variation: f32,
    pub blade_width: f32,
    pub render_radius: f32,
    pub render_lod: bool,
    pub brightness: f32,
    pub hue_jitter_degrees: f32,
    pub wind_speed: f32,
    pub wind_direction_degrees: f32,
    pub dynamic_wind: bool,
    pub dynamic_wind_strength: f32,
    pub deformation_strength: f32,
}

impl Default for ClientGrassSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            blades_per_block: 32,
            sparsity: 0.10,
            blade_height: 0.44,
            height_variation: 0.35,
            blade_width: 0.95,
            render_radius: 96.0,
            render_lod: true,
            brightness: 1.0,
            hue_jitter_degrees: 8.0,
            wind_speed: 1.0,
            wind_direction_degrees: 0.0,
            dynamic_wind: true,
            dynamic_wind_strength: 0.8,
            deformation_strength: 1.0,
        }
    }
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct ClientGrassSettingsChanged {
    pub previous: ClientGrassSettings,
    pub current: ClientGrassSettings,
    pub geometry_changed: bool,
}

pub trait ClientGrassSettingsApi: Send + Sync + 'static {}
