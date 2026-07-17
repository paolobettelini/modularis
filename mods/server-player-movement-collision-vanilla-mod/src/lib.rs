use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use block_shape_api::{BlockShape, BlockShapeApi, BlockShapeService};
use player_block_collision_api::resolve_player_collision;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_player_flight_api::{ServerPlayerFlightApi, ServerPlayerFlightCapabilities};
use server_player_flight_speed_api::{ServerPlayerFlightSpeedApi, ServerPlayerFlightSpeeds};
use server_player_hitbox_api::{
    ServerPlayerHitboxApi, ServerPlayerHitboxSet, ServerPlayerHitboxes,
};
use server_player_registry_api::{
    PendingServerPlayerMoves, ServerPlayerMovementSet, ServerPlayerRegistryApi,
};
use server_player_speed_api::{ServerPlayerSpeedApi, ServerPlayerSpeeds};
use std::marker::PhantomData;
use tokio::task::JoinHandle;
use voxel_math_api::BlockPos;

const MAX_PLAYER_MOVE_DELTA: f32 = 2.0;

pub struct ServerPlayerMovementCollisionVanillaMod<B>(PhantomData<B>);

impl<B: BlockManagerApi> ServerPlayerMovementCollisionVanillaMod<B> {
    pub fn init<
        W: ServerChunkWorldApi,
        P: ServerPlayerRegistryApi,
        S: ServerPlayerSpeedApi,
        F: ServerPlayerFlightApi,
        FS: ServerPlayerFlightSpeedApi,
        HB: ServerPlayerHitboxApi,
        H: BlockShapeApi,
    >(
        bevy: &mut BevyMod,
        _blocks: &mut B,
        _world_api: &mut W,
        _players: &mut P,
        _speed: &mut S,
        _flight: &mut F,
        _flight_speed: &mut FS,
        _hitboxes: &mut HB,
        _shapes: &mut H,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            validate_player_movement_collision::<B>
                .in_set(ServerPlayerMovementSet::Validate)
                .after(ServerPlayerHitboxSet),
        );
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn validate_player_movement_collision<B: BlockManagerApi>(
    world: Res<ServerChunkWorld>,
    speeds: Res<ServerPlayerSpeeds>,
    flight_capabilities: Res<ServerPlayerFlightCapabilities>,
    flight_speeds: Res<ServerPlayerFlightSpeeds>,
    hitboxes: Res<ServerPlayerHitboxes>,
    shapes: Res<BlockShapeService>,
    mut moves: ResMut<PendingServerPlayerMoves>,
) {
    for movement in &mut moves.moves {
        if movement.rejected {
            continue;
        }
        let mut allowed_speed = speeds.multiplier(movement.player_id);
        if flight_capabilities.enabled(movement.player_id) {
            allowed_speed = allowed_speed.max(flight_speeds.multiplier(movement.player_id));
        }
        let requested = clamp_requested_movement(
            movement.current_position,
            movement.accepted_position,
            allowed_speed,
        );
        let delta = requested - movement.current_position;
        let hitbox = hitboxes.hitbox(movement.player_id);
        movement.accepted_position = resolve_player_collision(
            movement.current_position,
            delta,
            hitbox.radius,
            hitbox.height,
            &|position| collision_shape::<B>(&world, &shapes, movement.player_id, position),
        )
        .position;
    }
}

fn clamp_requested_movement(current: Vec3, requested: Vec3, speed_multiplier: f32) -> Vec3 {
    let movement = requested - current;
    let distance = movement.length();
    let maximum = MAX_PLAYER_MOVE_DELTA * speed_multiplier.max(0.0);
    if distance <= maximum {
        requested
    } else if maximum <= f32::EPSILON {
        current
    } else {
        current + movement / distance * maximum
    }
}

fn collision_shape<B: BlockManagerApi>(
    world: &ServerChunkWorld,
    shapes: &BlockShapeService,
    player_id: player_network_message_types::PlayerId,
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
