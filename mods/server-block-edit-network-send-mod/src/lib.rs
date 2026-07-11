use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::{ServerBlockBroken, ServerBlockEditSet, ServerBlockPlaced};
use block_edit_events_mod::BlockEditEventsMod;
use block_edit_network_message_types::{BlockBrokenPacket, BlockPlacedPacket};
use generated_network_messages::ClientBoundMessage;
use player_network_message_types::PlayerId;
use server_chunk_world_api::{ServerChunkWorld, ServerChunkWorldApi};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use tokio::task::JoinHandle;
use voxel_math_api::ChunkPos;
use world_instance_api::WorldScopeId;

pub struct ServerBlockEditNetworkSendMod;

impl ServerBlockEditNetworkSendMod {
    pub fn init<N: ServerNetworkEventsApi, W: ServerChunkWorldApi, P: ServerPlayerRegistryApi>(
        bevy: &mut BevyMod,
        _events: &mut BlockEditEventsMod,
        _network_events: &mut N,
        _world: &mut W,
        _players: &mut P,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            broadcast_block_edits.in_set(ServerBlockEditSet::Sync),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn broadcast_block_edits(
    mut broken: MessageReader<ServerBlockBroken>,
    mut placed: MessageReader<ServerBlockPlaced>,
    world: Res<ServerChunkWorld>,
    players: Res<ServerPlayerRegistry>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    for event in broken.read() {
        packets.write(ServerPacketOut {
            audience: ServerAudience::Players(instance_viewers(
                &world,
                &players,
                &event.scope,
                event.position.chunk(),
            )),
            message: ClientBoundMessage::BlockBrokenPacket(BlockBrokenPacket {
                position: event.position,
                previous: event.previous.clone(),
            }),
        });
    }
    for event in placed.read() {
        packets.write(ServerPacketOut {
            audience: ServerAudience::Players(instance_viewers(
                &world,
                &players,
                &event.scope,
                event.position.chunk(),
            )),
            message: ClientBoundMessage::BlockPlacedPacket(BlockPlacedPacket {
                position: event.position,
                block: event.block.clone(),
                replaced: event.replaced.clone(),
            }),
        });
    }
}

fn instance_viewers(
    world: &ServerChunkWorld,
    players: &ServerPlayerRegistry,
    scope: &WorldScopeId,
    position: ChunkPos,
) -> Vec<PlayerId> {
    players
        .players()
        .into_iter()
        .filter(|player| {
            world
                .resident_key_for_player(player.id, position)
                .is_some_and(|key| &key.scope() == scope)
        })
        .map(|player| player.id)
        .collect()
}
