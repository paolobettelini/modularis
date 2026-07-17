use bevy::prelude::*;

pub use player_hitbox_api::{PLAYER_EYE_HEIGHT, PLAYER_HEIGHT, PLAYER_RADIUS};

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PlayerVelocity(pub Vec3);

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PreviousPlayerPosition(pub Vec3);

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Grounded(pub bool);

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerControllerSet {
    Input,
    MovementModifiers,
    ApplyMovementIntent,
    GravityForces,
    JumpForces,
    Forces,
    ForceOverrides,
    Movement,
    PostMovement,
    CameraSync,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct PlayerMovementConfig {
    pub walk_speed: f32,
}

impl Default for PlayerMovementConfig {
    fn default() -> Self {
        Self { walk_speed: 5.0 }
    }
}

/// Mutable ECS contract for optional movement feature mods.
///
/// The controller writes the base intent in `PlayerControllerSet::Input`.
/// Sprint, status effects or custom movement rules can modify it in
/// `MovementModifiers` before the controller applies it.
#[derive(Resource, Debug, Clone, Copy)]
pub struct PlayerPlanarMovementIntent {
    pub direction: Vec3,
    /// Base movement speed selected by the active movement mode.
    pub target_speed: f32,
    pub speed_multiplier: f32,
}

impl Default for PlayerPlanarMovementIntent {
    fn default() -> Self {
        Self {
            direction: Vec3::ZERO,
            target_speed: 0.0,
            speed_multiplier: 1.0,
        }
    }
}

pub trait PlayerControllerApi: Send + Sync + 'static {}
