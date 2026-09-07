use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_player_registry_api::*;
use server_player_surface_lib::*;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use block_manager_api::BlockManagerApi;
use block_shape_api::{BlockShapeApi, BlockShapeService};
use server_player_gravity_api::{ServerPlayerGravities, ServerPlayerGravityApi};
use server_player_hitbox_api::{ServerPlayerHitboxApi, ServerPlayerHitboxes};
use tokio::task::JoinHandle;

pub struct ServerPlayerSurfaceMotionVanillaMod;

impl ServerPlayerSurfaceMotionVanillaMod {
    pub fn init<B: BlockManagerApi>(
        bevy: &mut BevyMod,
        _world: &mut impl ServerChunkWorldApi,
        _players: &mut impl ServerPlayerRegistryApi,
        _gravity: &mut impl ServerPlayerGravityApi,
        _hitbox: &mut impl ServerPlayerHitboxApi,
        _blocks: &mut B,
        _shapes: &mut impl BlockShapeApi,
    ) -> Self {
        bevy.app
            .init_resource::<ServerSurfaceAnchors>()
            .configure_sets(
                Update,
                (
                    collision_api::CharacterCollisionSet::Rebase,
                    collision_api::CharacterCollisionSet::Resolve,
                )
                    .chain()
                    .in_set(ServerPlayerMovementSet::Validate),
            )
            .add_systems(
                Update,
                (
                    rebase_moves.in_set(collision_api::CharacterCollisionSet::Rebase),
                    observe::<B>
                        .in_set(ServerPlayerSurfaceSet::Observe)
                        .after(ServerPlayerMovementSet::Apply)
                        .before(ServerPlayerRelocationSet::Apply),
                ),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn rebase_moves(
    world: Res<ServerChunkWorld>,
    anchors: Res<ServerSurfaceAnchors>,
    mut moves: ResMut<PendingServerPlayerMoves>,
) {
    for movement in &mut moves.moves {
        if !movement.rejected {
            if let Some(anchor) = anchors.0.get(&movement.player_id) {
                rebase(&world, anchor, movement);
            }
        }
    }
}

fn observe<B: BlockManagerApi>(
    world: Res<ServerChunkWorld>,
    players: Res<ServerPlayerRegistry>,
    gravity: Res<ServerPlayerGravities>,
    hitboxes: Res<ServerPlayerHitboxes>,
    shapes: Res<BlockShapeService>,
    mut anchors: ResMut<ServerSurfaceAnchors>,
) {
    anchors.0.retain(|id, _| players.player(*id).is_some());

    for player in players.players() {
        let mut position = Vec3::from_array(player.position);
        if let Some(old) = anchors.0.get(&player.id).filter(|a| {
            a.epoch == players.movement_epoch(player.id).unwrap_or(0)
                && a.world_foot == position
        }) {
            if let Some(pose) = world.frames().transform(&old.scope, old.frame) {
                position = pose.local_to_world(old.local_foot.as_dvec3()).as_vec3();
            }
        }

        let hitbox = hitboxes.hitbox(player.id);
        let query = collision_api::CharacterQuery::new(
            position,
            Vec3::ZERO,
            player_gravity_api::gravity_up(gravity.gravity(player.id)),
            hitbox.radius,
            hitbox.height,
        );
        let geometry =
            server_player_movement_collision_lib::geometry::<B>(&world, &shapes, player.id, position);
        let hit = character_collision_lib::support(query, &geometry);
        let Some(hit) = hit.filter(|h| h.surface != 0) else {
            anchors.0.remove(&player.id);
            continue;
        };

        let local = voxel_math_api::BlockPos::new(
            position.x.floor() as i32,
            position.y.floor() as i32,
            position.z.floor() as i32,
        );
        let Some(key) = world.resident_key_for_player(player.id, local.chunk()) else {
            continue;
        };
        let frame = voxel_frame_api::VoxelFrameId::from_u128(hit.surface);
        let Some(pose) = world.frames().transform(&key.scope(), frame) else {
            continue;
        };
        anchors.0.insert(
            player.id,
            ServerSurfaceAnchor {
                scope: key.scope(),
                frame,
                local_foot: pose.world_to_local(position.as_dvec3()).as_vec3(),
                world_foot: Vec3::from_array(player.position),
                epoch: players.movement_epoch(player.id).unwrap_or(0),
            },
        );
    }
}
