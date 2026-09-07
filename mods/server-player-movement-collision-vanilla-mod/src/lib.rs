use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use block_shape_api::{BlockShapeApi, BlockShapeService};
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_player_flight_api::{ServerPlayerFlightApi, ServerPlayerFlightCapabilities};
use server_player_flight_speed_api::{ServerPlayerFlightSpeedApi, ServerPlayerFlightSpeeds};
use server_player_hitbox_api::{
    ServerPlayerHitboxApi, ServerPlayerHitboxSet, ServerPlayerHitboxes,
};
use server_player_movement_collision_lib::{
    DEFAULT_MAX_PLAYER_MOVE_DELTA, resolve_server_player_movement_result,
};
use server_player_registry_api::{
    PendingServerPlayerMoves, ServerPlayerMovementSet, ServerPlayerRegistryApi,
};
use server_player_speed_api::{ServerPlayerSpeedApi, ServerPlayerSpeeds};
use std::marker::PhantomData;
use tokio::task::JoinHandle;

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
        _gravity:&mut impl server_player_gravity_api::ServerPlayerGravityApi,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            validate_player_movement_collision::<B>
                .in_set(ServerPlayerMovementSet::Validate).in_set(collision_api::CharacterCollisionSet::Resolve)
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
    gravities:Res<server_player_gravity_api::ServerPlayerGravities>,
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
        let hitbox = hitboxes.hitbox(movement.player_id);
        let requested = movement.accepted_position;
        let result = resolve_server_player_movement_result::<B>(
            &world,
            &shapes,
            movement.player_id,
            movement.current_position,
            movement.accepted_position,
            hitbox.radius,
            hitbox.height,
            player_gravity_api::gravity_up(gravities.gravity(movement.player_id)),
            allowed_speed,
            DEFAULT_MAX_PLAYER_MOVE_DELTA,
        );
        movement.accepted_position = result.position;
        let up = player_gravity_api::gravity_up(gravities.gravity(movement.player_id));
        movement.authoritative_support_surface = result
            .support
            .map(|support| support.surface)
            .or_else(|| {
                result
                    .contacts
                    .iter()
                    .filter(|contact| contact.normal.dot(up) >= 0.707_106_77)
                    .max_by(|left, right| {
                        left.normal
                            .dot(up)
                            .total_cmp(&right.normal.dot(up))
                    })
                    .map(|contact| contact.surface)
            });
        if movement.accepted_position.distance_squared(requested) > 0.03_f32.powi(2) {
            debug!(player = movement.player_id, from = ?movement.current_position,
                requested = ?requested, accepted = ?movement.accepted_position,
                gravity = ?gravities.gravity(movement.player_id),
                "movement changed by collision/speed validation");
        }
    }
}
