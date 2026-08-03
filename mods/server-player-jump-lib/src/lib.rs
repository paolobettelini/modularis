use bevy::prelude::*;
use block_manager_api::BlockManagerApi;
use block_shape_api::{BlockShape, BlockShapeService};
use player_block_collision_api::collides_at;
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
    collides_at(
        position + gravity_direction * 0.05,
        radius,
        height,
        &|position| {
            world
                .block_for_player(player_id, position)
                .map_or_else(BlockShape::empty, |block| {
                    if B::is_solid(block.block) {
                        shapes.shape(&block)
                    } else {
                        BlockShape::empty()
                    }
                })
        },
    )
}
