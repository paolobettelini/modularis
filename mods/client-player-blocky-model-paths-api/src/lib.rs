use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct ClientPlayerBlockyModelPaths {
    pub model_path: &'static str,
    pub texture_path: Option<&'static str>,
    pub texture_size: Option<UVec2>,
    pub idle_animation_path: &'static str,
    pub walk_animation_path: Option<&'static str>,
    pub model_scale: f32,
    pub primitive_scale: f32,
    pub yaw_offset_radians: f32,
    pub vertical_animation_locked_nodes: &'static [&'static str],
}

pub trait ClientPlayerBlockyModelPathsApi: Send + Sync + 'static {}
