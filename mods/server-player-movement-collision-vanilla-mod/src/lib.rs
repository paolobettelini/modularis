use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_manager_api::{BlockId, BlockManagerApi};
use player_block_collision_api::resolve_player_collision;
use player_hitbox_api::{PLAYER_HEIGHT, PLAYER_RADIUS};
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_player_registry_api::{
    PendingServerPlayerMoves, ServerPlayerMovementSet, ServerPlayerRegistryApi,
};
use std::marker::PhantomData;
use tokio::task::JoinHandle;
use voxel_math_api::BlockPos;

const MAX_PLAYER_MOVE_DELTA: f32 = 2.0;

pub struct ServerPlayerMovementCollisionVanillaMod<B>(PhantomData<B>);

impl<B: BlockManagerApi> ServerPlayerMovementCollisionVanillaMod<B> {
    pub fn init<W: ServerChunkWorldApi, P: ServerPlayerRegistryApi>(
        bevy: &mut BevyMod,
        _blocks: &mut B,
        _world_api: &mut W,
        _players: &mut P,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            validate_player_movement_collision::<B>.in_set(ServerPlayerMovementSet::Validate),
        );
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn validate_player_movement_collision<B: BlockManagerApi>(
    world: Res<ServerChunkWorld>,
    mut moves: ResMut<PendingServerPlayerMoves>,
) {
    for movement in &mut moves.moves {
        if movement.rejected {
            continue;
        }
        let requested =
            clamp_requested_movement(movement.current_position, movement.accepted_position);
        let delta = requested - movement.current_position;
        movement.accepted_position = resolve_player_collision(
            movement.current_position,
            delta,
            PLAYER_RADIUS,
            PLAYER_HEIGHT,
            &|position| solid_block::<B>(&world, movement.player_id, position),
        )
        .position;
    }
}

fn clamp_requested_movement(current: Vec3, requested: Vec3) -> Vec3 {
    let movement = requested - current;
    let distance = movement.length();
    if distance <= MAX_PLAYER_MOVE_DELTA {
        requested
    } else {
        current + movement / distance * MAX_PLAYER_MOVE_DELTA
    }
}

fn solid_block<B: BlockManagerApi>(
    world: &ServerChunkWorld,
    player_id: player_network_message_types::PlayerId,
    position: BlockPos,
) -> bool {
    let Some(block) = world.block_for_player(player_id, position) else {
        return false;
    };
    block.block != BlockId::Air && B::is_solid(block.block)
}
