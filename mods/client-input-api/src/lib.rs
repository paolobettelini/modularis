use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub look_delta: Vec2,
    pub break_block_pressed: bool,
    pub break_block_held: bool,
    pub use_item_pressed: bool,
}

/// Capture is the deterministic boundary between the selected input backend
/// and feature mods which consume the current frame's intentions.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientInputSet {
    Capture,
}

pub trait InputApi: Send + Sync + 'static {}
