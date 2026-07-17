use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use block_shape_api::{BlockShape, BlockShapeApi, BlockShapeService};
use generated_network_messages::{NetworkMessageSet, PlayerJumpRequestReceived};
use network_protocol_mod::NetworkProtocolMod;
use player_block_collision_api::collides_at;
use player_gravity_api::{gravity_direction, gravity_up};
use player_hitbox_api::{PLAYER_HEIGHT, PLAYER_RADIUS};
use player_jump_api::JumpConfig;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_player_gravity_api::{ServerPlayerGravities, ServerPlayerGravityApi};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use std::marker::PhantomData;
use tokio::task::JoinHandle;

pub struct ServerPlayerJumpVanillaMod<B>(PhantomData<B>);

impl<B: BlockManagerApi> ServerPlayerJumpVanillaMod<B> {
    pub fn init<
        W: ServerChunkWorldApi,
        P: ServerPlayerRegistryApi,
        G: ServerPlayerGravityApi,
        H: BlockShapeApi,
    >(
        bevy: &mut BevyMod,
        _blocks: &mut B,
        _world: &mut W,
        _players: &mut P,
        _gravity: &mut G,
        _shapes: &mut H,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.init_resource::<JumpConfig>().add_systems(
            Update,
            handle_jump_requests::<B>.after(NetworkMessageSet::DispatchPackets),
        );
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn handle_jump_requests<B: BlockManagerApi>(
    gravities: Res<ServerPlayerGravities>,
    jump: Res<JumpConfig>,
    world: Res<ServerChunkWorld>,
    registry: Res<ServerPlayerRegistry>,
    shapes: Res<BlockShapeService>,
    mut requests: MessageReader<PlayerJumpRequestReceived>,
) {
    for request in requests.read() {
        let Some(player) = registry.player_for_address(request.source) else {
            continue;
        };
        let gravity = gravities.gravity(player.id);
        let up = gravity_up(gravity);
        let direction = gravity_direction(gravity);
        if direction.length_squared() == 0.0 {
            continue;
        }
        let position = Vec3::from_array(player.position);
        if !is_grounded::<B>(&world, &shapes, player.id, position, direction) {
            debug!(
                "ignored airborne jump request for player {} at {:?}",
                player.id, position
            );
            continue;
        }
        debug!(
            "accepted jump request for player {} with speed {} along {:?}",
            player.id, jump.speed, up
        );
    }
}

fn is_grounded<B: BlockManagerApi>(
    world: &ServerChunkWorld,
    shapes: &BlockShapeService,
    player_id: player_network_message_types::PlayerId,
    position: Vec3,
    gravity_direction: Vec3,
) -> bool {
    collides_at(
        position + gravity_direction * 0.05,
        PLAYER_RADIUS,
        PLAYER_HEIGHT,
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
