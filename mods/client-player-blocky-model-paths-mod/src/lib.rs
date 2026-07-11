use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_player_blocky_model_paths_api::{
    ClientPlayerBlockyModelPaths, ClientPlayerBlockyModelPathsApi,
};
use tokio::task::JoinHandle;

pub struct ClientPlayerBlockyModelPathsMod;

/// Multiplies the whole spawned player model after Blocky primitives have been
/// converted to world units.
pub const PLAYER_BLOCKY_MODEL_SCALE: f32 = 1.0;

/// Converts Hytale/Blockbench primitive units into Bevy world units.
pub const PLAYER_BLOCKY_PRIMITIVE_SCALE: f32 = 1.0 / 64.0;

/// Rotates the imported model around the player's local up axis so its visual
/// forward matches the gameplay/camera yaw convention.
pub const PLAYER_BLOCKY_YAW_OFFSET_RADIANS: f32 = 0.0;

/// Node names whose animation position must not move along the world's vertical
/// axis. Hytale walk clips often put a strong up/down locomotion offset on
/// `Pelvis`; the gameplay position already comes from the network transform.
pub const PLAYER_BLOCKY_VERTICAL_ANIMATION_LOCKED_NODES: &[&str] = &["Pelvis"];

impl ClientPlayerBlockyModelPathsMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.insert_resource(ClientPlayerBlockyModelPaths {
            model_path: "client-player-blocky-model-paths-mod/player.blockymodel",
            texture_path: Some("client-player-blocky-model-paths-mod/Outlander_1.png"),
            texture_size: Some(UVec2::new(256, 128)),
            idle_animation_path: "client-player-blocky-model-paths-mod/idle.blockyanim",
            walk_animation_path: Some("client-player-blocky-model-paths-mod/walk.blockyanim"),
            model_scale: PLAYER_BLOCKY_MODEL_SCALE,
            primitive_scale: PLAYER_BLOCKY_PRIMITIVE_SCALE,
            yaw_offset_radians: PLAYER_BLOCKY_YAW_OFFSET_RADIANS,
            vertical_animation_locked_nodes: PLAYER_BLOCKY_VERTICAL_ANIMATION_LOCKED_NODES,
        });
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientPlayerBlockyModelPathsApi for ClientPlayerBlockyModelPathsMod {}
