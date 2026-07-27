use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use block_shape_api::{BlockShapeApi, BlockShapeService};
use generated_network_messages::{NetworkMessageSet, PlayerJumpRequestReceived};
use network_protocol_mod::NetworkProtocolMod;
use player_jump_api::JumpConfig;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_player_gravity_api::{ServerPlayerGravities, ServerPlayerGravityApi};
use server_player_hitbox_api::{
    ServerPlayerHitboxApi, ServerPlayerHitboxSet, ServerPlayerHitboxes,
};
use server_player_jump_lib::validate_server_jump;
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use std::marker::PhantomData;
use tokio::task::JoinHandle;

pub struct ServerPlayerJumpVanillaMod<B>(PhantomData<B>);

impl<B: BlockManagerApi> ServerPlayerJumpVanillaMod<B> {
    pub fn init<
        W: ServerChunkWorldApi,
        P: ServerPlayerRegistryApi,
        G: ServerPlayerGravityApi,
        HB: ServerPlayerHitboxApi,
        H: BlockShapeApi,
    >(
        bevy: &mut BevyMod,
        _blocks: &mut B,
        _world: &mut W,
        _players: &mut P,
        _gravity: &mut G,
        _hitbox: &mut HB,
        _shapes: &mut H,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.init_resource::<JumpConfig>().add_systems(
            Update,
            handle_jump_requests::<B>
                .after(NetworkMessageSet::DispatchPackets)
                .after(ServerPlayerHitboxSet),
        );
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn handle_jump_requests<B: BlockManagerApi>(
    gravities: Res<ServerPlayerGravities>,
    hitboxes: Res<ServerPlayerHitboxes>,
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
        let position = Vec3::from_array(player.position);
        let hitbox = hitboxes.hitbox(player.id);
        let Some(validated) =
            validate_server_jump::<B>(&world, &shapes, player.id, position, gravity, hitbox, *jump)
        else {
            debug!(
                "ignored airborne jump request for player {} at {:?}",
                player.id, position
            );
            continue;
        };
        debug!(
            "accepted jump request for player {} with speed {} along {:?}",
            player.id, validated.speed, validated.direction
        );
    }
}
