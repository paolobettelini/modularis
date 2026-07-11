use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub look_delta: Vec2,
    pub break_block_pressed: bool,
    pub use_item_pressed: bool,
}

pub trait InputApi: Send + Sync + 'static {}
