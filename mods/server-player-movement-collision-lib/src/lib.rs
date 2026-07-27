use bevy::prelude::*;
use block_manager_api::BlockManagerApi;
use block_shape_api::{BlockShape, BlockShapeService};
use player_block_collision_api::resolve_player_collision;
use player_hitbox_api::PlayerHitbox;
use player_network_message_types::PlayerId;
use server_chunk_world_api::ServerChunkWorld;
use voxel_math_api::BlockPos;

pub const DEFAULT_MAX_PLAYER_MOVE_DELTA: f32 = 2.0;

/// Validates one movement against a chosen player's routed world.
///
/// This is intentionally a function rather than an always-running system. A
/// custom server can call it only in selected scopes, use another speed limit,
/// or replace collision completely.
pub fn resolve_server_player_movement<B: BlockManagerApi>(
    world: &ServerChunkWorld,
    shapes: &BlockShapeService,
    player_id: PlayerId,
    current: Vec3,
    requested: Vec3,
    hitbox: PlayerHitbox,
    speed_multiplier: f32,
    maximum_base_delta: f32,
) -> Vec3 {
    let requested =
        clamp_requested_movement(current, requested, speed_multiplier, maximum_base_delta);
    let delta = requested - current;
    resolve_player_collision(current, delta, hitbox.radius, hitbox.height, &|position| {
        collision_shape::<B>(world, shapes, player_id, position)
    })
    .position
}

pub fn clamp_requested_movement(
    current: Vec3,
    requested: Vec3,
    speed_multiplier: f32,
    maximum_base_delta: f32,
) -> Vec3 {
    let movement = requested - current;
    let distance = movement.length();
    let maximum = maximum_base_delta.max(0.0) * speed_multiplier.max(0.0);
    if distance <= maximum {
        requested
    } else if maximum <= f32::EPSILON {
        current
    } else {
        current + movement / distance * maximum
    }
}

pub fn collision_shape<B: BlockManagerApi>(
    world: &ServerChunkWorld,
    shapes: &BlockShapeService,
    player_id: PlayerId,
    position: BlockPos,
) -> BlockShape {
    let Some(block) = world.block_for_player(player_id, position) else {
        return BlockShape::empty();
    };
    if B::is_solid(block.block) {
        shapes.shape(&block)
    } else {
        BlockShape::empty()
    }
}
