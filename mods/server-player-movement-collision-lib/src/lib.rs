use bevy::prelude::*;
use block_manager_api::BlockManagerApi;
use block_shape_api::{BlockShape, BlockShapeService};

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
    hitbox_radius: f32,
    hitbox_height: f32,
    up: Vec3,
    speed_multiplier: f32,
    maximum_base_delta: f32,
) -> Vec3 {
    let requested =
        clamp_requested_movement(current, requested, speed_multiplier, maximum_base_delta);
    let delta = requested - current;
    let mut query=collision_api::CharacterQuery::new(current,delta,up,hitbox_radius,hitbox_height);
    let geometry=geometry::<B>(world,shapes,player_id,current);
    query.was_grounded=character_collision_lib::support(query,&geometry).is_some();
    character_collision_lib::resolve(query,&geometry).position
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

pub fn geometry<'a,B:BlockManagerApi>(world:&'a ServerChunkWorld,shapes:&'a BlockShapeService,player_id:PlayerId,position:Vec3)->impl collision_api::CharacterGeometry+'a {
 let scope=world.resident_key_for_player(player_id,BlockPos::new(position.x.floor() as i32,position.y.floor() as i32,position.z.floor() as i32).chunk()).map(|key|key.scope());
 voxel_frame_collision_lib::VoxelGeometry{
  shape:move |address:voxel_frame_api::VoxelBlockAddress|world.block_for_player(player_id,address).map_or_else(BlockShape::full_cube,|block|if B::is_solid(block.block){shapes.shape(&block)}else{BlockShape::empty()}),
  frames:move |bounds|scope.as_ref().map(|scope|world.frames().query(scope,bounds)).unwrap_or_default(),
 }
}
