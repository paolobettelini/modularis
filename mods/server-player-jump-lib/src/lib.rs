use bevy::prelude::*;
use block_manager_api::BlockManagerApi;
use block_shape_api::{BlockShape, BlockShapeService};

use player_gravity_api::{gravity_direction, gravity_up};
use player_network_message_types::PlayerId;
use server_chunk_world_api::ServerChunkWorld;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ValidatedServerJump {
    pub direction: Vec3,
    pub speed: f32,
}

/// Validates the reusable vanilla jump rule for one player.
///
/// It does not subscribe to input and does not mutate player state. A custom
/// server can invoke it conditionally, combine it with another rule, or ignore
/// it and implement a different jump model.
pub fn validate_server_jump<B: BlockManagerApi>(
    world: &ServerChunkWorld,
    shapes: &BlockShapeService,
    player_id: PlayerId,
    position: Vec3,
    gravity: Vec3,
    hitbox_radius: f32,
    hitbox_height: f32,
    jump_speed: f32,
) -> Option<ValidatedServerJump> {
    let up = gravity_up(gravity);
    let direction = gravity_direction(gravity);
    if direction.length_squared() == 0.0
        || !is_grounded::<B>(
            world,
            shapes,
            player_id,
            position,
            direction,
            hitbox_radius,
            hitbox_height,
        )
    {
        return None;
    }
    Some(ValidatedServerJump {
        direction: up,
        speed: jump_speed,
    })
}

pub fn is_grounded<B: BlockManagerApi>(
    world: &ServerChunkWorld,
    shapes: &BlockShapeService,
    player_id: PlayerId,
    position: Vec3,
    gravity_direction: Vec3,
    radius: f32,
    height: f32,
) -> bool {
    let geometry=server_player_movement_collision_lib::geometry::<B>(world,shapes,player_id,position);
    let query=collision_api::CharacterQuery::new(position,Vec3::ZERO,-gravity_direction,radius,height);
    character_collision_lib::support(query,&geometry).is_some()
}
