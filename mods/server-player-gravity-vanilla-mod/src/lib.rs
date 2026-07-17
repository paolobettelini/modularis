use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use player_network_message_types::PlayerId;
use server_chunk_world_api::ServerChunkWorldApi;
use server_network_api::ServerNetworkApi;
use server_player_gravity_api::ServerPlayerGravityApi;
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::ServerPlayerRegistryApi;
use std::{collections::HashMap, marker::PhantomData};
use tokio::task::JoinHandle;

#[derive(Resource, Default)]
pub struct ServerGravityVelocities {
    pub by_player: HashMap<PlayerId, Vec3>,
}

pub struct ServerPlayerGravityVanillaMod<B>(PhantomData<B>);

impl<B: BlockManagerApi> ServerPlayerGravityVanillaMod<B> {
    pub fn init<
        W: ServerChunkWorldApi,
        N: ServerNetworkApi,
        P: ServerPlayerRegistryApi,
        G: ServerPlayerGravityApi,
    >(
        bevy: &mut BevyMod,
        _blocks: &mut B,
        _world: &mut W,
        _network: &mut N,
        _players: &mut P,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _gravity: &mut G,
    ) -> Self {
        bevy.app
            .init_resource::<ServerGravityVelocities>()
            .add_systems(Update, forget_left_players);
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn forget_left_players(
    mut left: MessageReader<ServerPlayerLeft>,
    mut velocities: ResMut<ServerGravityVelocities>,
) {
    for left in left.read() {
        velocities.by_player.remove(&left.player_id);
    }
}
